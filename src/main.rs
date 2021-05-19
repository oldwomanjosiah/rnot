extern crate tokio;
extern crate tracing;
extern crate tracing_subscriber;
extern crate dbus;
extern crate dbus_tokio;
extern crate dbus_crossroads;
extern crate anyhow;


use std::{future::Future, marker::PhantomData};

use dbus::{channel::MatchingReceiver, message::MatchRule, strings::Member};
use dbus_crossroads::{Context, Crossroads};
use tracing::{debug, debug_span};
use anyhow::{Context as AContext, Result as AResult};

const DEFAULT_LOG: &'static str = "debug";
const DEFAULT_BUS_NAME: &'static str = "com.rnot.dbustest";

mod dbus_methods;

#[tokio::main]
async fn main() -> AResult<()> {

    // Set up tracing information
    {
        use tracing_subscriber::EnvFilter;
        let filter_layer = EnvFilter::try_from_default_env()
            .or_else(move |_| EnvFilter::try_new(DEFAULT_LOG))
            .unwrap();

        tracing_subscriber::fmt()
            //.with_writer(|| file_handle.clone())
            .with_env_filter(filter_layer)
            .init();
    }

    let setup_span = debug_span!("Initalization");
    let runni_span = debug_span!("Running");
    let clean_span = debug_span!("Cleanup");


    let _s = setup_span.enter();
    // Perform setup

    let close = std::sync::Arc::new(tokio::sync::Notify::new());

    let (resource, c) = dbus_tokio::connection::new_session_sync().context("Connecting to dbus")?;

    // Tell the program to exit in the case that we lost connection to dbus
    tokio::spawn({
        let close = close.clone();
        async move {
        let err = resource.await;
        tracing::error!("Error! Lost connection to dbus with error: {}", err);
        close.notify_one();
    }});

    // Request name from bus
    c.request_name(DEFAULT_BUS_NAME, false, true, false)
        .await
        .context("Could not get name from dbus")?;

    // Build out server spec
    let mut cr = Crossroads::new();

    cr.set_async_support(Some((c.clone(), Box::new(|x| { tokio::spawn(x); }))));

    let iface_token = cr.register(DEFAULT_BUS_NAME, |b| {
        // interface builder
        use dbus_methods::*;

        // Required dbus methods for org.freedesktop.Notification
        GetCapabilitiesMethod::register(b);
        ReceiveNotificationMethod::register(b);
        CloseNotificationMethod::register(b);
        GetServerInformationMethod::register(b);

        NotificationClosedSignal::register(b);
        ActionInvokedSignal::register(b);
    });

    cr.insert("/org/freedesktop/notify", &[iface_token], ());

    drop(_s); let _s = runni_span.enter();

    cr.set_add_standard_ifaces(true);

    // Run
    c.start_receive(MatchRule::new_method_call(), Box::new(move |msg, conn| {
        cr.handle_message(msg, conn)
            .ok().context("Could not handle incoming message from dbus")
            .unwrap();
        true
    }));

    // Wait to recieve the close signal from dbus
    close.notified().await;
    let _s = clean_span.enter();
    // Perform cleanup


    Ok(())
}
