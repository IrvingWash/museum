use std::path::PathBuf;

use eframe::{
    NativeOptions,
    egui::{self, ScrollArea, Ui},
};
use museum::{backup_manager::BackupManager, music_backuper::MusicBackuper};

pub fn run() {
    eframe::run_native(
        "museum",
        NativeOptions::default(),
        Box::new(|_| Ok(Box::<AppModel>::default())),
    )
    .expect("Failed to initialize egui");
}

#[derive(Default)]
struct AppModel {
    backup_model: Option<BackupModel>,
    manage_model: Option<ManageModel>,
}

impl eframe::App for AppModel {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match (&mut self.backup_model, &mut self.manage_model) {
                (None, None) => {
                    if ui.button("Backup").clicked() {
                        self.backup_model = Some(BackupModel::default());
                    }
                    if ui.button("Manage").clicked() {
                        self.manage_model = Some(ManageModel::default());
                    }
                }
                (Some(backup_model), None) => {
                    backup_model.update(ui);

                    if backup_model.is_done {
                        self.backup_model = None;
                    }
                }
                (None, Some(manage_model)) => {
                    manage_model.update(ui);

                    if manage_model.is_done {
                        self.manage_model = None;
                    }
                }
                _ => {}
            };
        });
    }
}

#[derive(Default)]
struct BackupModel {
    music_path: Option<PathBuf>,
    is_done: bool,
    error: Option<String>,
}

impl BackupModel {
    fn update(&mut self, ui: &mut Ui) {
        if let Some(error) = &self.error {
            ui.label(format!("Error: {}", error));
        }

        if ui.button("Back").clicked() {
            self.is_done = true;
        }

        if ui.button("Set music path").clicked() {
            if let Some(music_path) = rfd::FileDialog::new().pick_folder() {
                self.music_path = Some(music_path);
            }
        }

        if let Some(music_path) = &self.music_path {
            ui.label(format!("Music path: {}", music_path.to_string_lossy()));

            if ui.button("Save").clicked() {
                self.error = None;

                if let Some(backup_path) = rfd::FileDialog::new()
                    .set_file_name("music_backup.json")
                    .save_file()
                {
                    let mut backuper =
                        MusicBackuper::new(music_path.clone(), backup_path, Vec::new());

                    match backuper.backup() {
                        Ok(_) => {}
                        Err(e) => {
                            self.error = Some(e);
                            return;
                        }
                    };

                    match backuper.save() {
                        Ok(_) => {}
                        Err(e) => {
                            self.error = Some(e);
                            return;
                        }
                    }

                    self.is_done = true;
                }
            }
        }
    }
}

#[derive(Default)]
struct ManageModel {
    manager: Option<BackupManager>,
    is_done: bool,
    error: Option<String>,
    skip: String,
}

impl ManageModel {
    fn update(&mut self, ui: &mut Ui) {
        if ui.button("Back").clicked() {
            self.is_done = true;
        }

        match &mut self.manager {
            None => {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("JSON", &[".json"])
                    .pick_file()
                {
                    self.manager = Some(BackupManager::new(path).expect("Oops"));
                }
            }
            Some(manager) => {
                let label = ui.label("Skip to:");
                ui.text_edit_singleline(&mut self.skip)
                    .labelled_by(label.id);

                let mut backup = manager.get_backup().clone();
                let position = backup
                    .iter()
                    .position(|entry| entry.artist.starts_with(&self.skip));

                match position {
                    Some(position) => backup = backup.split_off(position),
                    None => {}
                }

                ScrollArea::vertical().show(ui, |ui| {
                    for entry in backup {
                        ui.label(format!("{}", entry));

                        if ui.button("Toggle").clicked() {
                            manager.toggle_downloaded(entry.id);

                            match manager.save() {
                                Ok(_) => {
                                    self.error = None;
                                }
                                Err(e) => self.error = Some(e),
                            }
                        }
                    }
                });
            }
        }
    }
}
