slite
=====

slack-lite client. 
The official slack client is pretty fat. I think it should be possible to make something very performant and lightweight. 

Rough idea:
* Use slack RTM API (this is what they use for their own clients anyway, and what slack-term presumably uses)
* use an SQL database for local data storage
* Application is then split into two layers: 
** layer 1 'receiver' subscribes to the slack RTM API and updates the SQLite database.
** layer 2 'transformer' transforms the data into a live-updating 'viewmodel' 
** layer 3 'presenter' builds a view from the viewmodel

* Writes work in a similar way... 
** layer 2 'action' accepts an command from the UI and inserts into the database marked as pending
** layer 1 'transmitter' checks for pending actions, makes the relevant API calls and updates accordingly

Intending to use 
* Rust, there's an existing RTM API client
* Sqlite3, it's lightweight, there's a rust library, it can do notification with update hooks.
* GTK+ for the UI layer probably (I can use the Glade designer, hook into events, it's got a text editor)
** alternativelly could use a TUI, there's plenty for rust


