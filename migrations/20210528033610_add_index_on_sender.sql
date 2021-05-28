-- Add migration script here

-- Allow Quicker Grouping by Sender
CREATE INDEX NotificationSenders ON Notification(SenderId);

-- Make sure that a single sender is never added more than once
CREATE UNIQUE INDEX SendersNames ON Sender(Name);
