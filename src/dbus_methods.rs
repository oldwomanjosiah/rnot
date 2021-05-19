use std::{future::Future, marker::PhantomData};

use dbus_crossroads::{Context, Crossroads, IfaceBuilder};
use tracing::debug;

pub struct GetCapabilitiesMethod;

impl GetCapabilitiesMethod {
    pub fn construct(mut ctx: Context, cr: &mut Crossroads, _: ()) -> impl Future<Output = PhantomData<(Vec<&'static str>,)>> {
        
        async move {
            ctx.reply(Ok((vec![],)))
        }
    }

    pub fn register(builder: &mut IfaceBuilder<()>) {
        builder.method_with_cr_async(
            "GetCapabilities",
            (),
            ("capabilities",),
            Self::construct,
        );
    }
}

/**
 * dbus method for receiving notifications from other applications
 */
pub struct ReceiveNotificationMethod;

impl ReceiveNotificationMethod {
    pub fn construct(mut ctx: Context, cr: &mut Crossroads, 

        (sender, replaces, icon, summary, body, actions, hints, timeout):

        // Types of dbus parameters
        (String, u32, String, String, String, Vec<String>, dbus::arg::PropMap, i32)) ->

        // Phantomdata for return types
        impl Future<Output = PhantomData<(String,)>> + Send + 'static {
        debug!("Responding to Notify");

        let s = format!("Hello {}", sender);

        // todo!("Send a 'message' to internal dispatch with notification info");
        async move {
           ctx.reply(Ok((s,)))
        }
    }

    pub fn register(builder: &mut IfaceBuilder<()>) {
        debug!("Registering to Notify");
        builder.method_with_cr_async(
            "Notify",
            ("app_name", "replaces_id", "app_icon", "summary", "body", "actions", "hints", "expire_timeout"),
            ("notification_id",),
            Self::construct
        );
    }

}

/**
 * dbus method for closing existing notifications
 */
pub struct CloseNotificationMethod;

impl CloseNotificationMethod {
    pub fn construct(mut ctx: Context, cr: &mut Crossroads,
                     (notification_id,): (u32,))
                            -> impl Future<Output = PhantomData<()>> {
        debug!("Responding to CloseNotification");

        todo!("Send a 'message' to the internal dispatch");
        async move {
            ctx.reply(Ok(()))
        }
    }

    pub fn register(builder: &mut IfaceBuilder<()>) {
        debug!("Registering to CloseNotification");
        builder.method_with_cr_async(
            "CloseNotification",
            ("id",),
            (),
            Self::construct,
        );
    }
}

/**
 * dbus method for getting information about the current running server
 */
pub struct GetServerInformationMethod;

impl GetServerInformationMethod {
    pub fn construct(mut ctx: Context, cr: &mut Crossroads, _: ()) -> impl Future<Output = PhantomData<(
        &'static str, &'static str, &'static str, &'static str,
        )>> {
        debug!("Responding to GetServerInformation");

        async move {
            ctx.reply(Ok((
                env!("CARGO_BIN_NAME"),
                "oldwomanjosiah",
                env!("CARGO_PKG_VERSION"),
                "1.2"
            )))
        }
    }

    pub fn register(builder: &mut IfaceBuilder<()>) {
        debug!("Registering to GetServerInformation");
        builder.method_with_cr_async(
            "GetServerInformation",
            (),
            ("name", "vendor", "version", "spec_version"),
            Self::construct,
        );
    }
}

pub struct NotificationClosedSignal;

impl NotificationClosedSignal {
    pub fn register(builder: &mut IfaceBuilder<()>) {
        debug!("Registering to NotificationClosed signal");
        builder.signal::<(u32, u32), _>("NotificationClosed", ("id", "reason"));
    }
}

pub struct ActionInvokedSignal;

impl ActionInvokedSignal {
    pub fn register(builder: &mut IfaceBuilder<()>) {
        debug!("Registering to ActionInvolked signal");
        builder.signal::<(u32, String), _>("ActionInvoked", ("id", "action_key"));
    }
}
