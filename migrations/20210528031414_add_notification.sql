-- Add migration script here

PRAGMA foreign_keys = ON;

-- Keep the sender separately so that we can cache the icon location if found
CREATE TABLE Sender (

	SenderId INTEGER PRIMARY KEY,

	Name TEXT NOT NULL,
	IconPath TEXT DEFAULT NULL,

	-- Unix Time Stamp
	LastNotifiedUTS DATETIME NOT NULL

);

-- Store a list of notifications
CREATE TABLE Notification (

	NotificationId INTEGER PRIMARY KEY,
	SenderId INTEGER NOT NULL REFERENCES Sender(SenderId),

	Summary TEXT NOT NULL,
	FormatBody TEXT,

	Received DATETIME NOT NULL,
	Timeout INTEGER
);

-- Store a list of notification actions
CREATE TABLE Action (
	NotificationId INTEGER,
	ActionId INTEGER,

	ActionKey TEXT NOT NULL,
	ActionFormatSummary TEXT NOT NULL,

	FOREIGN KEY (NotificationId) REFERENCES Notification(NotificationId) ON DELETE CASCADE,
	PRIMARY KEY (NotificationId, ActionId)
);
