lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", ""),
        ("Your Desktop", ""),
        ("desk_tip", "Sinu töölauale saab selle ID ja parooliga ligi pääseda."),
        ("Password", ""),
        ("Ready", ""),
        ("Established", ""),
        ("connecting_status", "RustDeski võrguga ühendumine..."),
        ("Enable service", ""),
        ("Start service", ""),
        ("Service is running", ""),
        ("Service is not running", ""),
        ("not_ready_status", "Pole valmis. Palun kontrolli oma ühendust"),
        ("Control Remote Desktop", ""),
        ("Transfer file", ""),
        ("Connect", ""),
        ("Recent sessions", ""),
        ("Address book", ""),
        ("Confirmation", ""),
        ("TCP tunneling", ""),
        ("Remove", ""),
        ("Refresh random password", ""),
        ("Set your own password", ""),
        ("Enable keyboard/mouse", ""),
        ("Enable clipboard", ""),
        ("Enable file transfer", ""),
        ("Enable TCP tunneling", ""),
        ("IP Whitelisting", ""),
        ("ID/Relay Server", "ID-/releeserver"),
        ("Import server config", ""),
        ("Export Server Config", ""),
        ("Import server configuration successfully", ""),
        ("Export server configuration successfully", ""),
        ("Invalid server configuration", ""),
        ("Clipboard is empty", ""),
        ("Stop service", ""),
        ("Change ID", ""),
        ("Your new ID", ""),
        ("length %min% to %max%", ""),
        ("starts with a letter", ""),
        ("allowed characters", ""),
        ("id_change_tip", "Lubatud on vaid a-z, A-Z, 0-9 ja _ (alakriips) tähemärgid. Esimene täht peab olema a-z või A-Z. Pikkus vahemikus 6-16."),
        ("Website", ""),
        ("About", ""),
        ("Slogan_tip", "Loodud südamega selles kaootilises maailmas!"),
        ("Privacy Statement", ""),
        ("Mute", ""),
        ("Build Date", "Ehituskuupäev"),
        ("Version", ""),
        ("Home", ""),
        ("Audio Input", "Helisisend"),
        ("Enhancements", ""),
        ("Hardware Codec", "Riistvarakoodek"),
        ("Adaptive bitrate", ""),
        ("ID Server", "ID-server"),
        ("Relay Server", "Releeserver"),
        ("API Server", "Rakendusliidese server"),
        ("invalid_http", "peab algama: http:// või https://"),
        ("Invalid IP", ""),
        ("Invalid format", ""),
        ("server_not_support", "Pole veel serveri poolt toetatud"),
        ("Not available", ""),
        ("Too frequent", ""),
        ("Cancel", ""),
        ("Skip", ""),
        ("Close", ""),
        ("Retry", ""),
        ("OK", ""),
        ("Password Required", "Parool on nõutud"),
        ("Please enter your password", ""),
        ("Remember password", ""),
        ("Wrong Password", "Vale parool"),
        ("Do you want to enter again?", ""),
        ("Connection Error", "Ühenduse viga"),
        ("Error", ""),
        ("Reset by the peer", ""),
        ("Connecting...", ""),
        ("Connection in progress. Please wait.", ""),
        ("Please try 1 minute later", ""),
        ("Login Error", "Sisselogimise viga"),
        ("Successful", ""),
        ("Connected, waiting for image...", ""),
        ("Name", ""),
        ("Type", ""),
        ("Modified", ""),
        ("Size", ""),
        ("Show Hidden Files", "Kuva peidetud faile"),
        ("Receive", ""),
        ("Send", ""),
        ("Refresh File", "Värskenda faili"),
        ("Local", ""),
        ("Remote", ""),
        ("Remote Computer", "Kaugarvuti"),
        ("Local Computer", "Kohalik arvuti"),
        ("Confirm Delete", "Kinnita kustutamist"),
        ("Delete", ""),
        ("Properties", ""),
        ("Multi Select", "Mitmikvalik"),
        ("Select All", "Vali kõik"),
        ("Unselect All", "Tühista kõigi valik"),
        ("Empty Directory", "Tühi kaust"),
        ("Not an empty directory", ""),
        ("Are you sure you want to delete this file?", ""),
        ("Are you sure you want to delete this empty directory?", ""),
        ("Are you sure you want to delete the file of this directory?", ""),
        ("Do this for all conflicts", ""),
        ("This is irreversible!", ""),
        ("Deleting", ""),
        ("files", ""),
        ("Waiting", ""),
        ("Finished", ""),
        ("Speed", ""),
        ("Custom Image Quality", "Kohandatud pildikvaliteet"),
        ("Privacy mode", ""),
        ("Block user input", ""),
        ("Unblock user input", ""),
        ("Adjust Window", "Kohanda akent"),
        ("Original", ""),
        ("Shrink", ""),
        ("Stretch", ""),
        ("Scrollbar", ""),
        ("ScrollAuto", ""),
        ("Good image quality", ""),
        ("Balanced", ""),
        ("Optimize reaction time", ""),
        ("Custom", ""),
        ("Show remote cursor", ""),
        ("Show quality monitor", ""),
        ("Disable clipboard", ""),
        ("Lock after session end", ""),
        ("Insert", ""),
        ("Insert Lock", "Sisesta lukk"),
        ("Refresh", ""),
        ("ID does not exist", ""),
        ("Failed to connect to rendezvous server", ""),
        ("Please try later", ""),
        ("Remote desktop is offline", ""),
        ("Key mismatch", ""),
        ("Timeout", ""),
        ("Failed to connect to relay server", ""),
        ("Failed to connect via rendezvous server", ""),
        ("Failed to connect via relay server", ""),
        ("Failed to make direct connection to remote desktop", ""),
        ("Set Password", "Määra parool"),
        ("OS Password", "Opsüsteemi parool"),
        ("install_tip", "Kasutajakonto kontrolli (UAC) tõttu ei saa RustDesk mõnel juhul korralikult kaugjuhtimispoolena töötada. Kontrolli vältimiseks palun klõpsa alloleval nupul, et RustDesk oma süsteemi paigaldada."),
        ("Click to upgrade", ""),
        ("Click to download", ""),
        ("Click to update", ""),
        ("Configure", ""),
        ("config_acc", "Töölaua kaugjuhtimiseks tuleb RustDeskile anda \"juurdepääsetavuse\" õigused."),
        ("config_screen", "Töölaua kaugjuhtimiseks tuleb RustDeskile anda \"ekraanisalvestuse\" õigused."),
        ("Installing ...", ""),
        ("Install", ""),
        ("Installation", ""),
        ("Installation Path", "Paigaldustee"),
        ("Create start menu shortcuts", ""),
        ("Create desktop icon", ""),
        ("agreement_tip", "Paigalduse alustamisel nõustud litsentsilepinguga."),
        ("Accept and Install", "Nõustu ja paigalda"),
        ("End-user license agreement", ""),
        ("Generating ...", ""),
        ("Your installation is lower version.", ""),
        ("not_close_tcp_tip", "Ara sulge seda akent, kuni kasutad tunnelit"),
        ("Listening ...", ""),
        ("Remote Host", "Kaughost"),
        ("Remote Port", "Kaugport"),
        ("Action", ""),
        ("Add", ""),
        ("Local Port", "Kohalik port"),
        ("Local Address", "Kohalik aadress"),
        ("Change Local Port", "Muuda kohalikku porti"),
        ("setup_server_tip", "Kiirema ühenduse jaoks palun seadista oma server"),
        ("Too short, at least 6 characters.", ""),
        ("The confirmation is not identical.", ""),
        ("Permissions", ""),
        ("Accept", ""),
        ("Dismiss", ""),
        ("Disconnect", ""),
        ("Enable file copy and paste", ""),
        ("Connected", ""),
        ("Direct and encrypted connection", ""),
        ("Relayed and encrypted connection", ""),
        ("Direct and unencrypted connection", ""),
        ("Relayed and unencrypted connection", ""),
        ("Enter Remote ID", "Sisesta kaug-ID"),
        ("Enter your password", ""),
        ("Logging in...", ""),
        ("Enable RDP session sharing", ""),
        ("Auto Login", "Logi automaatselt sisse (Kehtib vaid valiku \"lukusta pärast seansi lõppu\" lubamisel)"),
        ("Enable direct IP access", ""),
        ("Rename", ""),
        ("Space", ""),
        ("Create desktop shortcut", ""),
        ("Change Path", "Muuda failiteed"),
        ("Create Folder", "Loo kaust"),
        ("Please enter the folder name", ""),
        ("Fix it", ""),
        ("Warning", ""),
        ("Login screen using Wayland is not supported", ""),
        ("Reboot required", ""),
        ("Unsupported display server", ""),
        ("x11 expected", ""),
        ("Port", ""),
        ("Settings", ""),
        ("Username", ""),
        ("Invalid port", ""),
        ("Closed manually by the peer", ""),
        ("Enable remote configuration modification", ""),
        ("Run without install", ""),
        ("Connect via relay", ""),
        ("Always connect via relay", ""),
        ("whitelist_tip", "Ainult lubamisloendis IP saab mulle ligi"),
        ("Login", ""),
        ("Verify", ""),
        ("Remember me", ""),
        ("Trust this device", ""),
        ("Verification code", ""),
        ("verification_tip", "Registreeritud e-posti aadressile on saadetud kinnituskood, sisselogimise jätkamiseks sisesta kinnituskood."),
        ("Logout", ""),
        ("Tags", ""),
        ("Search ID", ""),
        ("whitelist_sep", "Eraldatud koma, semikooloni, tühikute või uue reaga"),
        ("Add ID", ""),
        ("Add Tag", "Lisa silt"),
        ("Unselect all tags", ""),
        ("Network error", ""),
        ("Username missed", ""),
        ("Password missed", ""),
        ("Wrong credentials", "Vale kasutajanimi või parool"),
        ("The verification code is incorrect or has expired", ""),
        ("Edit Tag", "Muuda silti"),
        ("Forget Password", "Unusta parool"),
        ("Favorites", ""),
        ("Add to Favorites", "Lisa lemmikutesse"),
        ("Remove from Favorites", "Eemalda lemmikutest"),
        ("Empty", ""),
        ("Invalid folder name", ""),
        ("Socks5 Proxy", "Socks5 proksi"),
        ("Hostname", ""),
        ("Discovered", ""),
        ("install_daemon_tip", "Süsteemikäivitusel käivitamiseks tuleb paigaldada süsteemiteenus."),
        ("Remote ID", ""),
        ("Paste", ""),
        ("Paste here?", ""),
        ("Are you sure to close the connection?", "Kas soovid kindlasti ühenduse sulgeda?"),
        ("Download new version", ""),
        ("Touch mode", ""),
        ("Mouse mode", ""),
        ("One-Finger Tap", "Ühe sõrme koputus"),
        ("Left Mouse", "Vasak hiireklahv"),
        ("One-Long Tap", "Üks pikk koputus"),
        ("Two-Finger Tap", "Kahe sõrme koputus"),
        ("Right Mouse", "Parem hiireklahv"),
        ("One-Finger Move", "Üks sõrmeliigutus"),
        ("Double Tap & Move", "Topeltkoputus ja liigutus"),
        ("Mouse Drag", "Hiirega sikutamine"),
        ("Three-Finger vertically", "Kolm sõrme vertikaalselt"),
        ("Mouse Wheel", "Hiirerullik"),
        ("Two-Finger Move", "Kahe sõrme liigutus"),
        ("Canvas Move", "Lõuendi liigutus"),
        ("Pinch to Zoom", "Näpistus-suum"),
        ("Canvas Zoom", "Lõuendi suum"),
        ("Reset canvas", ""),
        ("No permission of file transfer", ""),
        ("Note", ""),
        ("Connection", ""),
        ("Share Screen", "Jaga ekraani"),
        ("Chat", ""),
        ("Total", ""),
        ("items", ""),
        ("Selected", ""),
        ("Screen Capture", "Ekraanisalvestus"),
        ("Input Control", "Sisendjuhtimine"),
        ("Audio Capture", "Helisalvestus"),
        ("File Connection", "Failiühendus"),
        ("Screen Connection", "Kuvaühendus"),
        ("Do you accept?", ""),
        ("Open System Setting", "Ava süsteemisätted"),
        ("How to get Android input permission?", ""),
        ("android_input_permission_tip1", "Selleks, et kaugseade saaks sinu Androidi seadet juhtida hiire või puute abil, pead andma RustDeskile \"juurdepääsetavuse\" loa."),
        ("android_input_permission_tip2", "Palun mine järgmisele süsteemiseadete lehele, leia ja sisesta [Paigaldatud teenused], lülita teenus [RustDesk Input] sisse."),
        ("android_new_connection_tip", "Saabunud on uus juhtimistaotlus, mis soovib sinu praegust seadet juhtida."),
        ("android_service_will_start_tip", "\"Ekraanisalvestuse\" lubamine käivitab teenuse automaatselt, lubades teistel seadetel sinu seadmesse ühendust taotleda."),
        ("android_stop_service_tip", "Teenuse sulgemine sulgeb automaatselt kõik loodud ühendused."),
        ("android_version_audio_tip", "Kasutatav Androidi versioon ei toeta helisalvestust, palun täienda Android 10 või uuemale versioonile."),
        ("android_start_service_tip", "Koputa [Alusta teenust] või anna [Ekraanisalvestuse] luba, et ekraanijagamisteenust alustada."),
        ("android_permission_may_not_change_tip", "Loodud ühenduste õigused ei pruugi muutuda enne taasühendamist koheselt."),
        ("Account", ""),
        ("Overwrite", ""),
        ("This file exists, skip or overwrite this file?", ""),
        ("Quit", ""),
        ("Help", ""),
        ("Failed", ""),
        ("Succeeded", ""),
        ("Someone turns on privacy mode, exit", ""),
        ("Unsupported", ""),
        ("Peer denied", ""),
        ("Please install plugins", ""),
        ("Peer exit", ""),
        ("Failed to turn off", ""),
        ("Turned off", ""),
        ("Language", ""),
        ("Keep RustDesk background service", ""),
        ("Ignore Battery Optimizations", "Ignoreeri akuoptimeerimisi"),
        ("android_open_battery_optimizations_tip", "Kui soovid selle funktsiooni keelata, palun mine järgmisele RustDeski rakenduse seadete lehele, leia ja sisesta [Aku], eemalda linnuke valikult [Piiramata]"),
        ("Start on boot", ""),
        ("Start the screen sharing service on boot, requires special permissions", ""),
        ("Connection not allowed", ""),
        ("Legacy mode", ""),
        ("Map mode", ""),
        ("Translate mode", ""),
        ("Use permanent password", ""),
        ("Use both passwords", ""),
        ("Set permanent password", ""),
        ("Enable remote restart", ""),
        ("Restart remote device", ""),
        ("Are you sure you want to restart", ""),
        ("Restarting remote device", ""),
        ("remote_restarting_tip", "Kaugseade taaskäivitub, palun sulge see sõnumikast ja ühendu mõne aja pärast uuesti püsiva parooliga."),
        ("Copied", ""),
        ("Exit Fullscreen", "Lahku täisekraanist"),
        ("Fullscreen", ""),
        ("Mobile Actions", "Mobiilitegevused"),
        ("Select Monitor", "Vali kuvar"),
        ("Control Actions", "Juhtimistegevused"),
        ("Display Settings", "Kuvasätted"),
        ("Ratio", ""),
        ("Image Quality", "Pildikvaliteet"),
        ("Scroll Style", "Kerimisstiil"),
        ("Show Toolbar", "Kuva tööriistariba"),
        ("Hide Toolbar", "Peida tööriistariba"),
        ("Direct Connection", "Otseühendus"),
        ("Relay Connection", "Releeühendus"),
        ("Secure Connection", "Turvaline ühendus"),
        ("Insecure Connection", "Ebaturvaline ühendus"),
        ("Scale original", ""),
        ("Scale adaptive", ""),
        ("General", ""),
        ("Security", ""),
        ("Theme", ""),
        ("Dark Theme", "Tume teema"),
        ("Light Theme", "Hele teema"),
        ("Dark", ""),
        ("Light", ""),
        ("Follow System", "Järgi süsteemi"),
        ("Enable hardware codec", ""),
        ("Unlock Security Settings", "Lukusta lahti turvasätted"),
        ("Enable audio", ""),
        ("Unlock Network Settings", "Lukusta lahti võrgusätted"),
        ("Server", ""),
        ("Direct IP Access", "Otsene IP-ligipääs"),
        ("Proxy", ""),
        ("Apply", ""),
        ("Disconnect all devices?", ""),
        ("Clear", ""),
        ("Audio Input Device", "Heli sisendseade"),
        ("Use IP Whitelisting", "Kasuta IP-lubamisloendit"),
        ("Network", ""),
        ("Pin Toolbar", "Kinnita tööriistariba"),
        ("Unpin Toolbar", "Eemalda tööriistariba kinnitus"),
        ("Recording", ""),
        ("Directory", ""),
        ("Automatically record incoming sessions", ""),
        ("Change", ""),
        ("Start session recording", ""),
        ("Stop session recording", ""),
        ("Enable recording session", ""),
        ("Enable LAN discovery", ""),
        ("Deny LAN discovery", ""),
        ("Write a message", ""),
        ("Prompt", ""),
        ("Please wait for confirmation of UAC...", ""),
        ("elevated_foreground_window_tip", "Kaugtöölaua praegune aken nõuab töötamiseks kõrgemaid õigusi, mistõttu ei saa see ajutiselt hiirt ja klaviatuuri kasutada. Võid kaugkasutajal paluda minimeerida praegune aken või klõpsata ühenduse haldamise aknas kõrgendatud loa nuppu. Selle probleemi vältimiseks on soovitatav kaugseadmesse tarkvara paigaldada."),
        ("Disconnected", ""),
        ("Other", ""),
        ("Confirm before closing multiple tabs", ""),
        ("Keyboard Settings", "Klaviatuurisätted"),
        ("Full Access", "Täielik ligipääs"),
        ("Screen Share", "Ekraanijagamine"),
        ("Wayland requires Ubuntu 21.04 or higher version.", ""),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", ""),
        ("JumpLink", "Kuva"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Palun vali jagatav ekraan (tegutse partneri poolel)."),
        ("Show RustDesk", ""),
        ("This PC", ""),
        ("or", ""),
        ("Continue with", ""),
        ("Elevate", ""),
        ("Zoom cursor", ""),
        ("Accept sessions via password", ""),
        ("Accept sessions via click", ""),
        ("Accept sessions via both", ""),
        ("Please wait for the remote side to accept your session request...", ""),
        ("One-time Password", "Ühekordne parool"),
        ("Use one-time password", ""),
        ("One-time password length", ""),
        ("Request access to your device", ""),
        ("Hide connection management window", ""),
        ("hide_cm_tip", "Luba varjamine ainult siis, kui parooliga seansse võetakse vastu ning kasutatakse püsivat parooli."),
        ("wayland_experiment_tip", "Waylandi tugi on katsetusjärgus, järelvalveta juurdepääsu vajadusel palun kasuta X11."),
        ("Right click to select tabs", ""),
        ("Skipped", ""),
        ("Add to address book", ""),
        ("Group", ""),
        ("Search", ""),
        ("Closed manually by web console", ""),
        ("Local keyboard type", ""),
        ("Select local keyboard type", ""),
        ("software_render_tip", "Kui kasutad Linuxis Nvidia graafikakaarti ja kaugaken sulgub kohe pärast ühendamist, võib aidata üleminek avatud lähtekoodiga Nouveau draiverile ja valida tarkvaraline renderdamise. Vajalik on tarkvara taaskäivitamine."),
        ("Always use software rendering", ""),
        ("config_input", "Kaugtöölaua klaviatuuriga juhtimiseks pead andma RustDeskile \"sisendi jälgimise\" õigused."),
        ("config_microphone", "Kaugelt rääkimiseks pead andma RustDeskile \"heli salvestamise\" õigused."),
        ("request_elevation_tip", "Sa võid kõrgendatud õigusi taotleda ka siis, kui keegi on kaugpoolel."),
        ("Wait", ""),
        ("Elevation Error", "Kõrgendatud õiguste viga"),
        ("Ask the remote user for authentication", ""),
        ("Choose this if the remote account is administrator", ""),
        ("Transmit the username and password of administrator", ""),
        ("still_click_uac_tip", "Kaugkasutaja peab siiski ise vajutama käitatud RustDeski kasutajakonto kontrollis (UAC) OK-nuppu."),
        ("Request Elevation", "Taotle kõrgendatud õigusi"),
        ("wait_accept_uac_tip", "Palun oota, kuni kaugkasutaja nõustub UAC-dialoogiga (kasutajakonto kontroll)."),
        ("Elevate successfully", ""),
        ("uppercase", ""),
        ("lowercase", ""),
        ("digit", ""),
        ("special character", ""),
        ("length>=8", ""),
        ("Weak", ""),
        ("Medium", ""),
        ("Strong", ""),
        ("Switch Sides", "Vaheta pooli"),
        ("Please confirm if you want to share your desktop?", ""),
        ("Display", ""),
        ("Default View Style", "Vaikimisi kuvastiil"),
        ("Default Scroll Style", "Vaikimisi kerimisstiil"),
        ("Default Image Quality", "Vaikimisi pildikvaliteet"),
        ("Default Codec", "Vaikimisi koodek"),
        ("Bitrate", ""),
        ("FPS", ""),
        ("Auto", ""),
        ("Other Default Options", "Teised vaikevalikud"),
        ("Voice call", ""),
        ("Text chat", ""),
        ("Stop voice call", ""),
        ("relay_hint_tip", "Otseühendust ei pruugi olla võimalik luua; võid proovida ühendust relee kaudu. Lisaks, kui soovid esimesel katsel kasutada releed, võid lisada ID-le järelliite \"/r\" või valida viimaste seansside kaardil - kui see on olemas - valiku \"Ühenda alati relee kaudu\"."),
        ("Reconnect", ""),
        ("Codec", ""),
        ("Resolution", ""),
        ("No transfers in progress", ""),
        ("Set one-time password length", ""),
        ("RDP Settings", "RDP seaded"),
        ("Sort by", ""),
        ("New Connection", "Uus ühendus"),
        ("Restore", ""),
        ("Minimize", ""),
        ("Maximize", ""),
        ("Your Device", "Sinu seade"),
        ("empty_recent_tip", "Ups, hiljutised seansid puuduvad!\nAeg uus planeerida."),
        ("empty_favorite_tip", "Ei ole veel ühtegi lemmikpartnerit?\nLeia keegi, kellega suhelda ja lisa ta oma lemmikute hulka!"),
        ("empty_lan_tip", "Oh ei, tundub, et me pole veel ühtegi partnerit avastanud."),
        ("empty_address_book_tip", "Oh ei, tundub et sinu aadressiraamatus ei ole hetkel ühtegi partnerit."),
        ("eg: admin", ""),
        ("Empty Username", "Tühi kasutajanimi"),
        ("Empty Password", "Tühi parool"),
        ("Me", ""),
        ("identical_file_tip", "See fail on partneri omaga identne."),
        ("show_monitors_tip", "Kuva kuvarid tööriistaribal"),
        ("View Mode", "Kuvarežiim"),
        ("login_linux_tip", "X-töölaua seansi lubamiseks pead sisse logima Linuxi kaugkontosse."),
        ("verify_rustdesk_password_tip", "Kinnita RustDeski parooli"),
        ("remember_account_tip", "Jäta see konto meelde"),
        ("os_account_desk_tip", "Seda kontot kasutatakse kaug-opsüsteemi sisselogimiseks ja töölaua seansi lubamiseks headless-režiimis."),
        ("OS Account", "Opsüsteemi konto"),
        ("another_user_login_title_tip", "Teine kasutaja on juba sisse logitud"),
        ("another_user_login_text_tip", "Ühenda lahti"),
        ("xorg_not_found_title_tip", "Xorg-i ei leitud"),
        ("xorg_not_found_text_tip", "Palun paigalda Xorg"),
        ("no_desktop_title_tip", "Töölaud pole saadaval"),
        ("no_desktop_text_tip", "Palun paigalda GNOME Desktop"),
        ("No need to elevate", ""),
        ("System Sound", "Süsteemiheli"),
        ("Default", ""),
        ("New RDP", ""),
        ("Fingerprint", ""),
        ("Copy Fingerprint", "Kopeeri sõrmejälg"),
        ("no fingerprints", "Sõrmejäljed puuduvad"),
        ("Select a peer", ""),
        ("Select peers", ""),
        ("Plugins", ""),
        ("Uninstall", ""),
        ("Update", ""),
        ("Enable", ""),
        ("Disable", ""),
        ("Options", ""),
        ("resolution_original_tip", "Originaalne eraldusvõime"),
        ("resolution_fit_local_tip", "Ühita kohaliku eraldusvõimega"),
        ("resolution_custom_tip", "Kohandatud eraldusvõime"),
        ("Collapse toolbar", ""),
        ("Accept and Elevate", "Luba kõrgemate õigustega"),
        ("accept_and_elevate_btn_tooltip", "Võta ühendus vastu ja anna kõrgemad UAC-õigused (kasutajakonto kontroll)."),
        ("clipboard_wait_response_timeout_tip", "Koopia vastuse ootamisel tekkis ajalõpp."),
        ("Incoming connection", ""),
        ("Outgoing connection", ""),
        ("Exit", ""),
        ("Open", ""),
        ("logout_tip", "Kas soovid kindlasti välja logida?"),
        ("Service", ""),
        ("Start", ""),
        ("Stop", ""),
        ("exceed_max_devices", "Oled saavutanud hallatavate seadmete maksimaalse arvu."),
        ("Sync with recent sessions", ""),
        ("Sort tags", ""),
        ("Open connection in new tab", ""),
        ("Move tab to new window", ""),
        ("Can not be empty", ""),
        ("Already exists", ""),
        ("Change Password", "Vaheta parooli"),
        ("Refresh Password", "Värskenda parool"),
        ("ID", ""),
        ("Grid View", "Ruudustikuvaade"),
        ("List View", "Loendivaade"),
        ("Select", ""),
        ("Toggle Tags", "Lülita silte"),
        ("pull_ab_failed_tip", "Aadressiraamatu värskendamine ebaõnnestus"),
        ("push_ab_failed_tip", "Aadressiraamatu sünkroonimine serveriga ebaõnnestus"),
        ("synced_peer_readded_tip", "Hiljutistel seanssidel olnud seadmed sünkroonitakse tagasi aadressiraamatusse."),
        ("Change Color", "Vaheta värvi"),
        ("Primary Color", "Põhivärv"),
        ("HSV Color", "HSV-värv"),
        ("Installation Successful!", "Paigaldus oli edukas!"),
        ("Installation failed!", ""),
        ("Reverse mouse wheel", ""),
        ("{} sessions", ""),
        ("scam_title", "Võid olla KELMUSE ohver!"),
        ("scam_text1", "Kui räägid telefoniga kellegagi, keda EI TUNNE ja EI USALDA, kes on palunud sul RustDeski kasutada ja teenus käivitada, ära jätka ning lõpeta kõne koheselt."),
        ("scam_text2", "Tõenäoliselt on tegemist petturiga, kes üritab sinu raha või muid privaatseid andmeid varastada."),
        ("Don't show again", ""),
        ("I Agree", ""),
        ("Decline", ""),
        ("Timeout in minutes", ""),
        ("auto_disconnect_option_tip", "Sissetulevate seansside automaatne sulgemine kasutaja mitteaktiivsuse korral"),
        ("Connection failed due to inactivity", "Mitteaktiivsuse tõttu automaatselt lahti ühendatud"),
        ("Check for software update on startup", ""),
        ("upgrade_rustdesk_server_pro_to_{}_tip", "Palun täienda RustDesk Server Pro versioonile {} või uuem!"),
        ("pull_group_failed_tip", "Grupi värskendamine ebaõnnestus"),
        ("Filter by intersection", ""),
        ("Remove wallpaper during incoming sessions", ""),
        ("Test", ""),
        ("display_is_plugged_out_msg", "See kuvar on välja lülitatud, lülita esmasele kuvarile."),
        ("No displays", ""),
        ("elevated_switch_display_msg", "Lülita ümber esmasele kuvarile, sest kõrgendatud kasutajarežiimis ei toetata mitut kuvarit."),
        ("Open in new window", ""),
        ("Show displays as individual windows", ""),
        ("Use all my displays for the remote session", ""),
        ("selinux_tip", "SELinux on su seadmes lubatud, mis võib RustDeskil takistada juhitud poolel toimimist."),
        ("Change view", ""),
        ("Big tiles", ""),
        ("Small tiles", ""),
        ("List", ""),
        ("Virtual display", ""),
        ("Plug out all", ""),
        ("True color (4:4:4)", ""),
        ("Enable blocking user input", ""),
        ("id_input_tip", "Võid sisestada ID, otsese IP või domeeni koos pordiga (<domeen>:<port>).\nKui soovid juurdepääsu seadmele mõnes teises serveris, lisa palun serveri aadress (<id>@<serveri_aadress>?key=<võtme_väärtus>), näiteks,\n9123456234@192.168.16.1:21117?key=5Qbwsde3unUcJBtrx9ZkvUmwFNoExHzpryHuPUdqlWM=.\nKui soovid juurdepääsu seadmele avalikus serveris, sisesta \"<id>@public\", avaliku serveri puhul ei ole võtit vaja."),
        ("privacy_mode_impl_mag_tip", "Režiim 1"),
        ("privacy_mode_impl_virtual_display_tip", "Režiim 2"),
        ("Enter privacy mode", ""),
        ("Exit privacy mode", ""),
        ("idd_not_support_under_win10_2004_tip", "Kaudse kuvari draiver ei ole toetatud. Vajalik on Windows 10, versioon 2004 või uuem."),
        ("switch_display_elevated_connections_tip", "Mitme ühenduse korral ei toetata kõrgendatud kasutajarežiimil üleminekut muule kui primaarsele kuvale. Kui soovid juhtida mitut ekraani, palun proovi uuesti pärast paigaldamist."),
        ("input_source_1_tip", "Sisendallikas 1"),
        ("input_source_2_tip", "Sisendallikas 2"),
        ("capture_display_elevated_connections_tip", "Mitme ekraani jäädvustamine ei ole kõrgendatud kasutajarežiimis toetatud. Kui soovid juhtida mitut ekraani, palun proovi uuesti pärast paigaldamist."),
        ("Swap control-command key", ""),
        ("swap-left-right-mouse", "Vaheta vasak ja parem hiirenupp"),
        ("2FA code", ""),
        ("More", ""),
        ("enable-2fa-title", ""),
        ("enable-2fa-desc", ""),
        ("wrong-2fa-code", ""),
        ("enter-2fa-title", ""),
        ("Email verification code must be 6 characters.", ""),
        ("2FA code must be 6 digits.", ""),
        ("Multiple Windows sessions found", ""),
        ("Please select the session you want to connect to", ""),
        ("powered_by_me", ""),
        ("outgoing_only_desk_tip", ""),
        ("preset_password_warning", ""),
        ("Security Alert", ""),
        ("My address book", ""),
        ("Personal", ""),
        ("Owner", ""),
        ("Set shared password", ""),
        ("Exist in", ""),
        ("Read-only", ""),
        ("Read/Write", ""),
        ("Full Control", ""),
        ("share_warning_tip", ""),
        ("Everyone", ""),
        ("ab_web_console_tip", ""),
    ].iter().cloned().collect();
}
