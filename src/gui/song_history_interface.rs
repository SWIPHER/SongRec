use app_dirs::{app_root, AppInfo, AppDataType::*};
use std::error::Error;
use std::path::PathBuf;
use gtk::prelude::*;

use crate::utils::csv_song_history::SongHistoryRecord;

/// This file contains code for interfacing between the CSV Song history
/// format defined within the "src/utils/csv_song_history.rs" file, the
/// GTK-rs GUI of SongRec and the filesystem while using the GUI.

pub struct SongHistoryInterface {
    csv_path: String,
    gtk_list_store: gtk::ListStore,
    chronological_records: Vec<SongHistoryRecord>
}

impl SongHistoryInterface {
    
    pub fn new(gtk_list_store: gtk::ListStore) -> Result<Self, Box<dyn Error>> {

        let mut interface = SongHistoryInterface {
            csv_path: SongHistoryInterface::obtain_csv_path()?,
            gtk_list_store: gtk_list_store,
            chronological_records: vec![]
        };
        
        interface.load()?;
        
        Ok(interface)
    }
    
    pub fn obtain_csv_path() -> Result<String, Box<dyn Error>> {
        let app_info = AppInfo {
            name: "SongRec",
            author: "SongRec"
        };
        
        let mut csv_path: PathBuf = app_root(UserData, &app_info)?; // Creates the application's local directory if necessary
        csv_path.push("song_history.csv");
        
        Ok(csv_path.to_str().unwrap().to_string())
        
    }
    
    /// All the code displaying the initial state of the song recognition
    /// history is stored here.
    
    fn load(self: &mut Self) -> Result<(), Box<dyn Error>> {
        match csv::ReaderBuilder::new()
        .flexible(true)
        .from_path(&self.csv_path) {
            Ok(mut reader) => {
                    
                for result in reader.deserialize() {
                    
                    let record: SongHistoryRecord = result?;

                    self.gtk_list_store.set(&self.gtk_list_store.insert(0), &[0, 1, 2], &[&record.song_name, &record.album, &record.recognition_date]);

                    self.chronological_records.push(record);
                                        
                };
            },
            _ => { } // File does not exists, ignore
        };
        Ok(())
    }
    
    pub fn wipe_and_save(self: &mut Self) {
        self.chronological_records.clear();
        
        self.gtk_list_store.clear();
        
        let mut writer = csv::Writer::from_path(&self.csv_path).unwrap();
        
        writer.flush().unwrap();
    }
    
    pub fn add_column_and_save(self: &mut Self, record: SongHistoryRecord) {
        self.gtk_list_store.set(&self.gtk_list_store.insert(0), &[0, 1, 2], &[&record.song_name, &record.album, &record.recognition_date]);

        self.chronological_records.push(record);
        
        let mut writer = csv::Writer::from_path(&self.csv_path).unwrap();
        
        for record in self.chronological_records.iter() {
            writer.serialize(record).unwrap();
        }
        
        writer.flush().unwrap();
    }
    
}
