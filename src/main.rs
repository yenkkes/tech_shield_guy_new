use crypto_layer::SecurityModuleError;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button, Label, ListBox, ListBoxRow};
use gtk4 as gtk;
use gtk4::Entry;

#[allow(unused_imports)]
use crypto_layer::common::{
    crypto::{
        algorithms::{
            encryption::{AsymmetricEncryption, BlockCiphers, EccCurves, EccSchemeAlgorithm},
            hashes::Hash,
            KeyBits,
        },
        KeyUsage,
    },
    traits::{key_handle::KeyHandle, module_provider::Provider},
};

// Import YubiKeyProvider and HsmProviderConfig for HSM operations
use crypto_layer::hsm::{yubikey::YubiKeyProvider, HsmProviderConfig};

#[cfg(feature = "yubi")]

static mut SIGNATURE: Vec<u8> = Vec::new();

fn main() -> glib::ExitCode {

    let application = Application::builder()
        .application_id("com.example.DemoApp")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Übersicht")
            .default_width(500)
            .default_height(400)
            .build();

        let list_box = ListBox::new(); // Erstelle eine neue ListBox
        let actions = vec![
            "Schlüssel generieren",
            "Daten verschlüsseln",
            "Daten entschlüsseln",
            "Daten signieren",
            "Signatur verifizieren",
        ];

        // ListBox füllen
        for action in actions {
            let label = Label::new(Some(action));
            let row = ListBoxRow::new();
            row.set_child(Some(&label));
            list_box.append(&row);
        }

        let app_clone = app.clone(); // Klone app für spätere Verwendung im Button Click Event

        // Aktion auswählen, wenn auf ein Listenelement geklickt wird
        list_box.connect_row_activated(move |_, row| {
            let index = row.index();
            match index {
                0 => create_new_window(&app_clone, "Schlüssel generieren".to_string()),
                1 => create_new_window(&app_clone, "Daten verschlüsseln".to_string()),
                2 => create_new_window(&app_clone, "Daten entschlüsseln".to_string()),
                3 => create_new_window(&app_clone, "Daten signieren".to_string()),
                4 => create_new_window(&app_clone, "Signatur verifizieren".to_string()),
                _ => {}
            }
        });

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
        vbox.append(&list_box);

        window.set_child(Some(&vbox));
        window.present();
    });

    application.run()
}

fn create_new_window(app: &Application, title: String) {
    if title != "Schlüssel generieren" {
let new_window = ApplicationWindow::builder()
        .application(app)
        .title(title.clone())
        .default_width(400)
        .default_height(300)
        .build();

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
    vbox.set_spacing(5);

    let label_key_id = Label::new(Some("Key ID:"));
    let entry_key_id = Entry::new();
    let entry_key_id_clone = entry_key_id.clone();

    let label_data = Label::new(Some("Daten:"));
    let entry_data = Entry::new();
    let entry_data_clone = entry_data.clone();

    let label_encryption_type = Label::new(Some("Verschlüsselungsart wählen:"));
    let combo_encryption_type = gtk::ComboBoxText::new();
    combo_encryption_type.append(None, "RSA1024");
    combo_encryption_type.append(None, "RSA2048");
    combo_encryption_type.append(None, "ECC256");
    combo_encryption_type.append(None, "ECC384");
    combo_encryption_type.set_active(Some(0)); // Standardmäßig den ersten Eintrag aktivieren
    let combo_encryption_type_clone = combo_encryption_type.clone();

    let button = Button::with_label("Aktion ausführen");
    let app2 = app.clone();
    button.connect_clicked(move |_| {
        let key_id = entry_key_id.text().to_string();
        let data = entry_data.text().to_string();
        let encryption_type = combo_encryption_type.active_text().unwrap().to_string();
        perform_action(&app2,&title, &data, &key_id, &encryption_type);
    });

    vbox.append(&label_key_id);
    vbox.append(&entry_key_id_clone);
    vbox.append(&label_data);
    vbox.append(&entry_data_clone);
    vbox.append(&label_encryption_type);
    vbox.append(&combo_encryption_type_clone);
    vbox.append(&button);

    new_window.set_child(Some(&vbox));
    new_window.present();
    } else {
    let new_window = ApplicationWindow::builder()
        .application(app)
        .title(title.clone())
        .default_width(400)
        .default_height(300)
        .build();

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
    vbox.set_spacing(5);

    let label_key_id = Label::new(Some("Key ID:"));
    let entry_key_id = Entry::new();
    let entry_key_id_clone = entry_key_id.clone();

    
    let label_encryption_type = Label::new(Some("Welchen Schlüssel wollen sie generieren?"));
    let combo_encryption_type = gtk::ComboBoxText::new();
    combo_encryption_type.append(None, "RSA1024");
    combo_encryption_type.append(None, "RSA2048");
    combo_encryption_type.append(None, "ECC256");
    combo_encryption_type.append(None, "ECC384");
    combo_encryption_type.set_active(Some(0)); // Standardmäßig den ersten Eintrag aktivieren
    let combo_encryption_type_clone = combo_encryption_type.clone();

    let button = Button::with_label("Generieren");
    let app2 = app.clone();
    
    let data = " ";

    button.connect_clicked(move |_| {
        let key_id = entry_key_id.text().to_string();
        let encryption_type = combo_encryption_type.active_text().unwrap().to_string();
        perform_action(&app2,&title, &data, &key_id, &encryption_type);
    });

    vbox.append(&label_key_id);
    vbox.append(&entry_key_id_clone);
    vbox.append(&label_encryption_type);
    vbox.append(&combo_encryption_type_clone);
    vbox.append(&button);

    new_window.set_child(Some(&vbox));
    new_window.present();
    }
    
}


fn perform_action(app: &Application, action: &str, data: &str, key_id: &str, encryption_type: &str) {
    match action {
        //   "Daten verschlüsseln" => encrypt_data(data, key_id, encryption_type),
        //   "Daten entschlüsseln" => decrypt_data(data, key_id, encryption_type),
        "Schlüssel generieren" => {
            generate(app, encryption_type, key_id);
        }
        "Daten signieren" => {
            let ergebnis = sign_data(data, key_id, encryption_type);
            match ergebnis {
                Ok(signat) => {
                    unsafe { SIGNATURE = signat };
                    let ausgabe = "Signatur erfolgreich";
                    create_new_window2(app, ausgabe.to_string());
                }
                Err(_) => {
                    let ausgabe = "Es konnte keine Signatur erstellt werden";
                    create_new_window2(app, ausgabe.to_string());
                }
            }
        }
        "Signatur verifizieren" => {
            let signature = unsafe { SIGNATURE.clone() };
            println!("Signatur: {:?}",  signature);
            let ergebnis = verify_signature(data, key_id, encryption_type, signature);
            match ergebnis {
                Ok(_) => {
                    let ausgabe = "Signatur erfolgreich verifiziert";
                    create_new_window2(app, ausgabe.to_string());
                },
                Err(_) => {
                    let ausgabe = "Signatur konnte nicht verifiziert werden";
                    create_new_window2(app, ausgabe.to_string());
                },
            }
        }
        _ => {}
    }
    println!(
        "Aktion: {}, Daten: {}, Key ID: {}, Verschlüsselungstyp: {}",
        action, data, key_id, encryption_type
    );
    // Führen Sie hier die entsprechende Aktion durch
}

fn verify_signature(
    data: &str,
    key_id: &str,
    encryption_type: &str,
    signature: Vec<u8>,
) -> Result<(), SecurityModuleError> {
    let mut provider = YubiKeyProvider::new(key_id.to_string());
    let mut config = HsmProviderConfig::new(
        AsymmetricEncryption::Rsa(KeyBits::Bits1024),
        vec![KeyUsage::SignEncrypt],
    );
    match encryption_type {
        "RSA1024" => {
            config = HsmProviderConfig::new(
                AsymmetricEncryption::Rsa(KeyBits::Bits1024),
                vec![KeyUsage::SignEncrypt],
            );
        }
        "RSA2048" => {
            config = HsmProviderConfig::new(
                AsymmetricEncryption::Rsa(KeyBits::Bits2048),
                vec![KeyUsage::SignEncrypt],
            );
        }
        "ECC256" => {
            config = HsmProviderConfig::new(
                AsymmetricEncryption::Ecc(EccSchemeAlgorithm::EcDsa(EccCurves::P256)),
                vec![KeyUsage::SignEncrypt],
            );
        }
        "ECC384" => {
            config = HsmProviderConfig::new(
                AsymmetricEncryption::Ecc(EccSchemeAlgorithm::EcDsa(EccCurves::P384)),
                vec![KeyUsage::SignEncrypt],
            );
        }
        _ => {}
    }

    provider
        .initialize_module()
        .expect("Failed to initialize module");

    provider
        .load_key(key_id, config)
        .expect("Failed to load key");

    let signature = signature.as_slice();
    let data = data.trim().as_bytes();
    let verify = provider.verify_signature(data, &signature);
    match verify {
        Ok(_) => Ok(()),
        Err(err) => {
            return Err(SecurityModuleError::SignatureVerificationError(
                err.to_string(),
            ))
        }
    }
}

fn sign_data(
    data: &str,
    key_id: &str,
    encryption_type: &str,
) -> Result<Vec<u8>, SecurityModuleError> {
    let mut provider = YubiKeyProvider::new(key_id.to_string());
    let mut config = HsmProviderConfig::new(
        AsymmetricEncryption::Rsa(KeyBits::Bits1024),
        vec![KeyUsage::SignEncrypt],
    );
    match encryption_type {
        "RSA1024" => {
            config = HsmProviderConfig::new(
                AsymmetricEncryption::Rsa(KeyBits::Bits1024),
                vec![KeyUsage::SignEncrypt],
            );
        }
        "RSA2048" => {
            config = HsmProviderConfig::new(
                AsymmetricEncryption::Rsa(KeyBits::Bits2048),
                vec![KeyUsage::SignEncrypt],
            );
        }
        "ECC256" => {
            config = HsmProviderConfig::new(
                AsymmetricEncryption::Ecc(EccSchemeAlgorithm::EcDsa(EccCurves::P256)),
                vec![KeyUsage::SignEncrypt],
            );
        }
        "ECC384" => {
            config = HsmProviderConfig::new(
                AsymmetricEncryption::Ecc(EccSchemeAlgorithm::EcDsa(EccCurves::P384)),
                vec![KeyUsage::SignEncrypt],
            );
        }
        _ => {}
    }

    provider
        .initialize_module()
        .expect("Failed to initialize module");
    provider
        .load_key(key_id, config)
        .expect("Failed to load key");
    let data: &[u8] = data.trim().as_bytes();
    let signature = provider.sign_data(data);
    match signature {
        Ok(sign) => Ok(sign),
        Err(err) => {
            return Err(SecurityModuleError::SignatureVerificationError(
                err.to_string(),
            ))
        }
    }
}

fn create_new_window2(app: &Application, message: String) {
    let new_window = ApplicationWindow::builder()
        .application(app)
        .title("Nachricht")
        .default_width(400)
        .default_height(300)
        .build();

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
    vbox.set_spacing(5);

    let label_message = Label::new(Some(&message));
    vbox.append(&label_message);

    new_window.set_child(Some(&vbox));
    new_window.present();
}

fn generate(app: &Application, encryption_type: &str, key_id: &str) {
    let mut provider = YubiKeyProvider::new(key_id.to_string());
    let mut config = HsmProviderConfig::new(
        AsymmetricEncryption::Rsa(KeyBits::Bits1024),
        vec![KeyUsage::SignEncrypt],
    );
    match encryption_type {
        "RSA1024" => {
            config = HsmProviderConfig::new(
                AsymmetricEncryption::Rsa(KeyBits::Bits1024),
                vec![KeyUsage::SignEncrypt],
            );
        }
        "RSA2048" => {
            config = HsmProviderConfig::new(
                AsymmetricEncryption::Rsa(KeyBits::Bits2048),
                vec![KeyUsage::SignEncrypt],
            );
        }
        "ECC256" => {
            config = HsmProviderConfig::new(
                AsymmetricEncryption::Ecc(EccSchemeAlgorithm::EcDsa(EccCurves::P256)),
                vec![KeyUsage::SignEncrypt],
            );
        }
        "ECC384" => {
            config = HsmProviderConfig::new(
                AsymmetricEncryption::Ecc(EccSchemeAlgorithm::EcDsa(EccCurves::P384)),
                vec![KeyUsage::SignEncrypt],
            );
        }
        _ => {}
    }

    provider
        .initialize_module()
        .expect("Failed to initialize module");
    match encryption_type {
        "RSA1024" => {
            let rsa = provider
                .create_key(key_id, config);
            match rsa {
                Ok(_) => {
                    let ausgabe = "RSA1024 Schlüssel erfolgreich generiert";
                    create_new_window2(app, ausgabe.to_string());
                }
                Err(_) => {
                    let ausgabe = "Fehler beim generieren des RSA1024 Schlüssels";
                    create_new_window2(app, ausgabe.to_string());
                }
            
            }
        }
        "RSA2048" => {
            let rsa = provider
                .create_key(key_id, config);
            match rsa {
                Ok(_) => {
                    let ausgabe = "RSA2048 Schlüssel erfolgreich generiert";
                    create_new_window2(app, ausgabe.to_string());
                }
                Err(_) => {
                    let ausgabe = "Fehler beim generieren des RSA2048 Schlüssels";
                    create_new_window2(app, ausgabe.to_string());
                }
            
            }
        }
        "ECC256" => {
            let ecc = provider
                .create_key(key_id, config);
            match ecc {
                Ok(_) => {
                    let ausgabe = "ECC256 Schlüssel erfolgreich generiert";
                    create_new_window2(app, ausgabe.to_string());
                }
                Err(_) => {
                    let ausgabe = "Fehler beim generieren des ECC256 Schlüssels";
                    create_new_window2(app, ausgabe.to_string());
                }
            
            }
        }
        "ECC384" => {
            let ecc = provider
                .create_key(key_id, config);
            match ecc {
                Ok(_) => {
                    let ausgabe = "ECC384 Schlüssel erfolgreich generiert";
                    create_new_window2(app, ausgabe.to_string());
                }
                Err(_) => {
                    let ausgabe = "Fehler beim generieren des ECC384 Schlüssels";
                    create_new_window2(app, ausgabe.to_string());
                }
            
            }
        }
        _ => {}
    }
}