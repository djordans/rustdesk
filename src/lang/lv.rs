lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Statuss"),
        ("Your Desktop", "Jūsu darbvirsma"),
        ("desk_tip", "Jūsu darbvirsmai var piekļūt ar šo ID un paroli."),
        ("Password", "Parole"),
        ("Ready", "Gatavs"),
        ("Established", "Izveidots"),
        ("connecting_status", "Notiek savienojuma izveide ar RustDesk tīklu..."),
        ("Enable Service", "Iespējot servisu"),
        ("Start Service", "Sākt servisu"),
        ("Service is running", "Pakalpojums darbojas"),
        ("Service is not running", "Pakalpojums nedarbojas"),
        ("not_ready_status", "Nav gatavs. Lūdzu, pārbaudiet savienojumu"),
        ("Control Remote Desktop", "Vadīt attālo darbvirsmu"),
        ("Transfer File", "Pārsūtīt failu"),
        ("Connect", "Savienoties"),
        ("Recent Sessions", "Pēdējās sesijas"),
        ("Address Book", "Adrešu grāmata"),
        ("Confirmation", "Apstiprinājums"),
        ("TCP Tunneling", "TCP tunelēšana"),
        ("Remove", "Noņemt"),
        ("Refresh random password", "Atsvaidzināt nejaušo paroli"),
        ("Set your own password", "Iestatiet savu paroli"),
        ("Enable Keyboard/Mouse", "Iespējot tastatūru/peli"),
        ("Enable Clipboard", "Iespējot starpliktuvi"),
        ("Enable File Transfer", "Iespējot failu pārsūtīšanu"),
        ("Enable TCP Tunneling", "Iespējot TCP tunelēšanu"),
        ("IP Whitelisting", "IP baltais saraksts"),
        ("ID/Relay Server", "ID/releja serveris"),
        ("Import Server Config", "Importēt servera konfigurāciju"),
        ("Export Server Config", "Eksportēt servera konfigurāciju"),
        ("Import server configuration successfully", "Servera konfigurācija veiksmīgi importēta"),
        ("Export server configuration successfully", "Servera konfigurācija veiksmīgi eksportēta"),
        ("Invalid server configuration", "Nederīga servera konfigurācija"),
        ("Clipboard is empty", "Starpliktuve ir tukša"),
        ("Stop service", "Apturēt servisu"),
        ("Change ID", "Mainīt ID"),
        ("Your new ID", "Jūsu jaunais ID"),
        ("length %min% to %max%", "garums %min% līdz %max%"),
        ("starts with a letter", "sākas ar burtu"),
        ("allowed characters", "atļautās rakstzīmes"),
        ("id_change_tip", "Atļautas tikai rakstzīmes a-z, A-Z, 0-9 un _ (pasvītrojums). Pirmajam burtam ir jābūt a-z, A-Z. Garums no 6 līdz 16."),
        ("Website", "Tīmekļa vietne"),
        ("About", "Par"),
        ("Slogan_tip", "Radīts ar sirdi šajā haotiskajā pasaulē!"),
        ("Privacy Statement", "Paziņojums par konfidencialitāti"),
        ("Mute", "Izslēgt skaņu"),
        ("Build Date", "Būvēšanas datums"),
        ("Version", "Versija"),
        ("Home", "Sākums"),
        ("Audio Input", "Audio ievade"),
        ("Enhancements", "Uzlabojumi"),
        ("Hardware Codec", "Aparatūras kodeks"),
        ("Adaptive bitrate", "Adaptīvais bitu pārraides ātrums"),
        ("ID Server", "ID serveris"),
        ("Relay Server", "Releja serveris"),
        ("API Server", "API serveris"),
        ("invalid_http", "jāsākas ar http:// vai https://"),
        ("Invalid IP", "Nederīga IP"),
        ("Invalid format", "Nederīgs formāts"),
        ("server_not_support", "Serveris vēl neatbalsta"),
        ("Not available", "Nav pieejams"),
        ("Too frequent", "Pārāk bieži"),
        ("Cancel", "Atcelt"),
        ("Skip", "Izlaist"),
        ("Close", "Aizvērt"),
        ("Retry", "Mēģināt vēlreiz"),
        ("OK", "Labi"),
        ("Password Required", "Nepieciešama parole"),
        ("Please enter your password", "Lūdzu, ievadiet paroli"),
        ("Remember password", "Atcerēties paroli"),
        ("Wrong Password", "Nepareiza parole"),
        ("Do you want to enter again?", "Vai vēlaties ievadīt vēlreiz?"),
        ("Connection Error", "Savienojuma kļūda"),
        ("Error", "Kļūda"),
        ("Reset by the peer", "Atiestatīts ar sesiju"),
        ("Connecting...", "Notiek savienojuma izveide..."),
        ("Connection in progress. Please wait.", "Notiek savienošanās. Lūdzu, uzgaidiet."),
        ("Please try 1 minute later", "Lūdzu, mēģiniet 1 minūti vēlāk"),
        ("Login Error", "Pieteikšanās kļūda"),
        ("Successful", "Veiksmīgi"),
        ("Connected, waiting for image...", "Savienots, gaida attēlu..."),
        ("Name", "Nosaukums"),
        ("Type", "Tips"),
        ("Modified", "Modificēšanas dat."),
        ("Size", "Lielums"),
        ("Show Hidden Files", "Rādīt slēptos failus"),
        ("Receive", "Saņemt"),
        ("Send", "Sūtīt"),
        ("Refresh File", "Atsvaidzināt failu"),
        ("Local", "Vietējais"),
        ("Remote", "Attālais"),
        ("Remote Computer", "Attālais dators"),
        ("Local Computer", "Vietējais dators"),
        ("Confirm Delete", "Apstiprināt dzēšanu"),
        ("Delete", "Dzēst"),
        ("Properties", "Rekvizīti"),
        ("Multi Select", "Vairākatlase"),
        ("Select All", "Atlasīt visu"),
        ("Unselect All", "Noņemt atlasi visiem"),
        ("Empty Directory", "Tukša direktorija"),
        ("Not an empty directory", "Nav tukša direktorija"),
        ("Are you sure you want to delete this file?", "Vai tiešām vēlaties dzēst šo failu?"),
        ("Are you sure you want to delete this empty directory?", "Vai tiešām vēlaties dzēst šo tukšo direktoriju?"),
        ("Are you sure you want to delete the file of this directory?", "Vai tiešām vēlaties dzēst šī direktorija failu?"),
        ("Do this for all conflicts", "Pielietot visiem konfliktiem"),
        ("This is irreversible!", "Tas ir neatgriezeniski!"),
        ("Deleting", "Dzēšana"),
        ("files", "faili"),
        ("Waiting", "Gaida"),
        ("Finished", "Pabeigts"),
        ("Speed", "Ātrums"),
        ("Custom Image Quality", "Pielāgota attēla kvalitāte"),
        ("Privacy mode", "Privātuma režīms"),
        ("Block user input", "Bloķēt lietotāja ievadi"),
        ("Unblock user input", "Atbloķēt lietotāja ievadi"),
        ("Adjust Window", "Pielāgot logu"),
        ("Original", "Oriģināls"),
        ("Shrink", "Saraut"),
        ("Stretch", "Izstiept"),
        ("Scrollbar", "Ritjosla"),
        ("ScrollAuto", "Ritināt automātiski"),
        ("Good image quality", "Laba attēla kvalitāte"),
        ("Balanced", "Sabalansēts"),
        ("Optimize reaction time", "Optimizēt reakcijas laiku"),
        ("Custom", "Pielāgots"),
        ("Show remote cursor", "Rādīt attālo kursoru"),
        ("Show quality monitor", "Rādīt kvalitātes monitoru"),
        ("Disable clipboard", "Atspējot starpliktuvi"),
        ("Lock after session end", "Bloķēt pēc sesijas beigām"),
        ("Insert", "Ievietot"),
        ("Insert Lock", "Ievietot Bloķēt"),
        ("Refresh", "Atsvaidzināt"),
        ("ID does not exist", "ID neeksistē"),
        ("Failed to connect to rendezvous server", "Neizdevās izveidot savienojumu ar starpposma serveri"),
        ("Please try later", "Lūdzu, mēģiniet vēlāk"),
        ("Remote desktop is offline", "Attālā darbvirsma ir bezsaistē"),
        ("Key mismatch", "Atslēgu neatbilstība"),
        ("Timeout", "Noildze"),
        ("Failed to connect to relay server", "Neizdevās izveidot savienojumu ar releja serveri"),
        ("Failed to connect via rendezvous server", "Neizdevās izveidot savienojumu, izmantojot starpposma serveri"),
        ("Failed to connect via relay server", "Neizdevās izveidot savienojumu, izmantojot releja serveri"),
        ("Failed to make direct connection to remote desktop", "Neizdevās izveidot tiešu savienojumu ar attālo darbvirsmu"),
        ("Set Password", "Uzstādīt paroli"),
        ("OS Password", "OS parole"),
        ("install_tip", "UAC dēļ RustDesk dažos gadījumos nevar pareizi darboties kā attālā puse. Lai izvairītos no UAC, lūdzu, noklikšķiniet uz tālāk esošās pogas, lai instalētu RustDesk sistēmā."),
        ("Click to upgrade", "Jaunināt"),
        ("Click to download", "Lejupielādēt"),
        ("Click to update", "Atjaunināt"),
        ("Configure", "Konfigurēt"),
        ("config_acc", "Lai attālināti vadītu savu darbvirsmu, jums ir jāpiešķir RustDesk \"Pieejamība\" atļaujas."),
        ("config_screen", "Lai attālināti piekļūtu darbvirsmai, jums ir jāpiešķir RustDesk \"Ekrāna tveršana\" atļaujas."),
        ("Installing ...", "Notiek instalēšana..."),
        ("Install", "Uzstādīt"),
        ("Installation", "Instalēšana"),
        ("Installation Path", "Instalācijas ceļš"),
        ("Create start menu shortcuts", "Izveidot sākuma izvēlnes īsceļus"),
        ("Create desktop icon", "Izveidot darbvirsmas ikonu"),
        ("agreement_tip", "Sākot instalēšanu, jūs piekrītat licences līgumam."),
        ("Accept and Install", "Pieņemt un instalēt"),
        ("End-user license agreement", "Gala lietotāja licences līgums"),
        ("Generating ...", "Ģenerēšana..."),
        ("Your installation is lower version.", "Jūsu instalācijai ir zemāka versija."),
        ("not_close_tcp_tip", "Neaizveriet šo logu, kamēr izmantojat tuneli"),
        ("Listening ...", "Klausās..."),
        ("Remote Host", "Attālais resursdators"),
        ("Remote Port", "Attālais ports"),
        ("Action", "Darbība"),
        ("Add", "Pievienot"),
        ("Local Port", "Vietējais ports"),
        ("Local Address", "Vietējā adrese"),
        ("Change Local Port", "Mainīt vietējo portu"),
        ("setup_server_tip", "Lai iegūtu ātrāku savienojumu, lūdzu, iestatiet savu serveri"),
        ("Too short, at least 6 characters.", "Pārāk īss, vismaz 6 rakstzīmes."),
        ("The confirmation is not identical.", "Apstiprinājums nav identisks."),
        ("Permissions", "Atļaujas"),
        ("Accept", "Pieņemt"),
        ("Dismiss", "Noraidīt"),
        ("Disconnect", "Atvienot"),
        ("Allow using keyboard and mouse", "Atļaut izmantot tastatūru un peli"),
        ("Allow using clipboard", "Atļaut izmantot starpliktuvi"),
        ("Allow hearing sound", "Atļaut klausīties skaņu"),
        ("Allow file copy and paste", "Atļaut failu kopēšanu un ielīmēšanu"),
        ("Connected", "Savienots"),
        ("Direct and encrypted connection", "Tiešs un šifrēts savienojums"),
        ("Relayed and encrypted connection", "Pārslēgts un šifrēts savienojums"),
        ("Direct and unencrypted connection", "Tiešs un nešifrēts savienojums"),
        ("Relayed and unencrypted connection", "Pārslēgts un nešifrēts savienojums"),
        ("Enter Remote ID", "Ievadiet attālo ID"),
        ("Enter your password", "Ievadiet savu paroli"),
        ("Logging in...", "Ielogoties..."),
        ("Enable RDP session sharing", "Iespējot RDP sesiju koplietošanu"),
        ("Auto Login", "Automātiskā pieteikšanās (derīga tikai tad, ja esat iestatījis \"Bloķēt pēc sesijas beigām\")"),
        ("Enable Direct IP Access", "Iespējot tiešo IP piekļuvi"),
        ("Rename", "Pārdēvēt"),
        ("Space", "Vieta"),
        ("Create Desktop Shortcut", "Izveidot saīsni uz darbvirsmas"),
        ("Change Path", "Mainīt ceļu"),
        ("Create Folder", "Izveidot mapi"),
        ("Please enter the folder name", "Lūdzu, ievadiet mapes nosaukumu"),
        ("Fix it", "Salabot to"),
        ("Warning", "Brīdinājums"),
        ("Login screen using Wayland is not supported", "Pieteikšanās ekrāns, izmantojot Wayland netiek atbalstīts"),
        ("Reboot required", "Nepieciešama restartēšana"),
        ("Unsupported display server", "Neatbalstīts displeja serveris"),
        ("x11 expected", "x11 paredzams"),
        ("Port", "Ports"),
        ("Settings", "Iestatījumi"),
        ("Username", "Lietotājvārds"),
        ("Invalid port", "Nederīgs ports"),
        ("Closed manually by the peer", "Sesija aizvērta manuāli"),
        ("Enable remote configuration modification", "Iespējot attālās konfigurācijas modifikāciju"),
        ("Run without install", "Palaist bez instalēšanas"),
        ("Connect via relay", "Savienot, izmantojot releju"),
        ("Always connect via relay", "Vienmēr izveidot savienojumu, izmantojot releju"),
        ("whitelist_tip", "Man var piekļūt tikai baltajā sarakstā iekļautās IP adreses"),
        ("Login", "Pieslēgties"),
        ("Verify", "Pārbaudīt"),
        ("Remember me", "Atcerēties mani"),
        ("Trust this device", "Uzticēties šai ierīcei"),
        ("Verification code", "Verifikācijas kods"),
        ("verification_tip", "Verifikācijas kods ir nosūtīts uz reģistrēto e-pasta adresi, ievadiet verifikācijas kodu, lai turpinātu pieslēgšanos."),
        ("Logout", "Izlogoties"),
        ("Tags", "Tagi"),
        ("Search ID", "Meklēt ID"),
        ("whitelist_sep", "Atdalīts ar komatu, semikolu, atstarpēm vai jaunu rindiņu"),
        ("Add ID", "Pievienot ID"),
        ("Add Tag", "Pievienot tagu"),
        ("Unselect all tags", "Noņemt visu tagu atlasi"),
        ("Network error", "Tīkla kļūda"),
        ("Username missed", "Lietotājvārds ir izlaists"),
        ("Password missed", "Parole nav ievadīta"),
        ("Wrong credentials", "Nepareizs lietotājvārds vai parole"),
        ("The verification code is incorrect or has expired", "Verifikācijas kods ir nepareizs vai tam ir beidzies derīguma termiņš"),
        ("Edit Tag", "Rediģēt tagu"),
        ("Forget Password", "Neatcerēties paroli"),
        ("Favorites", "Izlase"),
        ("Add to Favorites", "Pievienot pie izlases"),
        ("Remove from Favorites", "Noņemt no izlases"),
        ("Empty", "Tukšs"),
        ("Invalid folder name", "Nederīgs mapes nosaukums"),
        ("Socks5 Proxy", "Socks5 starpniekserveris"),
        ("Hostname", "Resursdatora nosaukums"),
        ("Discovered", "Atklāts"),
        ("install_daemon_tip", "Lai palaistu pie startēšanas, ir jāinstalē sistēmas serviss."),
        ("Remote ID", "Attālais ID"),
        ("Paste", "Ielīmēt"),
        ("Paste here?", "Ielīmēt šeit?"),
        ("Are you sure to close the connection?", "Vai tiešām vēlaties aizvērt savienojumu?"),
        ("Download new version", "Lejupielādēt jauno versiju"),
        ("Touch mode", "Skārienrežīms"),
        ("Mouse mode", "Peles režīms"),
        ("One-Finger Tap", "Pieskāriens ar vienu pirkstu"),
        ("Left Mouse", "Kreisā pele"),
        ("One-Long Tap", "Viens garš pieskāriens"),
        ("Two-Finger Tap", "Pieskāriens ar diviem pirkstiem"),
        ("Right Mouse", "Labā pele"),
        ("One-Finger Move", "Viena pirksta pārvietošana"),
        ("Double Tap & Move", "Dubultskāriens & pārvietošana"),
        ("Mouse Drag", "Peles vilkšana"),
        ("Three-Finger vertically", "Trīs pirksti vertikāli"),
        ("Mouse Wheel", "Peles ritenis"),
        ("Two-Finger Move", "Divu pirkstu pārvietošana"),
        ("Canvas Move", "Audekla pārvietošana"),
        ("Pinch to Zoom", "Saspiediet, lai tuvinātu"),
        ("Canvas Zoom", "Audekla tālummaiņa"),
        ("Reset canvas", "Atiestatīt audeklu"),
        ("No permission of file transfer", "Nav atļaujas failu pārsūtīšanai"),
        ("Note", "Piezīme"),
        ("Connection", "Savienojums"),
        ("Share Screen", "Koplietot ekrānu"),
        ("Chat", "Tērzēšana"),
        ("Total", "Kopā"),
        ("items", "vienumi"),
        ("Selected", "Atlasīts"),
        ("Screen Capture", "Ekrāna tveršana"),
        ("Input Control", "Ievades vadība"),
        ("Audio Capture", "Audio tveršana"),
        ("File Connection", "Failu savienojums"),
        ("Screen Connection", "Ekrāna savienojums"),
        ("Do you accept?", "Vai Jūs pieņemat?"),
        ("Open System Setting", "Atvērt sistēmas iestatījumus"),
        ("How to get Android input permission?", "Kā iegūt Android ievades atļauju?"),
        ("android_input_permission_tip1", "Lai attālā ierīce varētu vadīt jūsu Android ierīci, izmantojot peli vai pieskārienu, jums ir jāatļauj RustDesk izmantot servisu \"Pieejamība\"."),
        ("android_input_permission_tip2", "Lūdzu, dodieties uz nākamo sistēmas iestatījumu lapu, atrodiet un atveriet [Instalētie pakalpojumi], ieslēdziet servisu [RustDesk Input]."),
        ("android_new_connection_tip", "Ir saņemts jauns vadības pieprasījums, kas vēlas kontrolēt jūsu pašreizējo ierīci."),
        ("android_service_will_start_tip", "Ieslēdzot \"Ekrāna tveršana\", serviss tiks automātiski startēts, ļaujot citām ierīcēm pieprasīt savienojumu ar jūsu ierīci."),
        ("android_stop_service_tip", "Pakalpojuma aizvēršana automātiski aizvērs visus izveidotos savienojumus."),
        ("android_version_audio_tip", "Pašreizējā Android versija neatbalsta audio uztveršanu, lūdzu, jauniniet uz Android 10 vai jaunāku versiju."),
        ("android_start_service_tip", "Pieskarieties [Sākt servisu] vai iespējojiet [Ekrāna tveršana] atļauju, lai sāktu ekrāna koplietošanas servisu."),
        ("android_permission_may_not_change_tip", "Izveidoto savienojumu atļaujas nevar mainīt uzreiz, kamēr nav atkārtoti izveidots savienojums."),
        ("Account", "Konts"),
        ("Overwrite", "Pārrakstīt"),
        ("This file exists, skip or overwrite this file?", "Šis fails pastāv, izlaist vai pārrakstīt šo failu?"),
        ("Quit", "Iziet"),
        ("Help", "Palīdzība"),
        ("Failed", "Neizdevās"),
        ("Succeeded", "Izdevās"),
        ("Someone turns on privacy mode, exit", "Kāds ieslēdza privātuma režīmu, iziet"),
        ("Unsupported", "Neatbalstīts"),
        ("Peer denied", "Sesija noraidīta"),
        ("Please install plugins", "Lūdzu, instalējiet spraudņus"),
        ("Peer exit", "Iziet no attālās ierīces"),
        ("Failed to turn off", "Neizdevās izslēgt"),
        ("Turned off", "Izslēgts"),
        ("In privacy mode", "Privātuma režīmā"),
        ("Out privacy mode", "Izslēgts privātuma režīms"),
        ("Language", "Valoda"),
        ("Keep RustDesk background service", "Saglabāt RustDesk fona servisu"),
        ("Ignore Battery Optimizations", "Ignorēt akumulatora optimizāciju"),
        ("android_open_battery_optimizations_tip", "Ja vēlaties atspējot šo funkciju, lūdzu, dodieties uz nākamo RustDesk lietojumprogrammas iestatījumu lapu, atrodiet un atveriet [Akumulators], noņemiet atzīmi no [Neierobežots]"),
        ("Start on Boot", "Palaist pie ieslēgšanas"),
        ("Start the screen sharing service on boot, requires special permissions", "Startējiet ekrāna koplietošanas pakalpojumu ieslēgšanas laikā, nepieciešamas īpašas atļaujas"),
        ("Connection not allowed", "Savienojums nav atļauts"),
        ("Legacy mode", "Novecojis režīms"),
        ("Map mode", "Kartēšanas režīms"),
        ("Translate mode", "Tulkošanas režīms"),
        ("Use permanent password", "Izmantot pastāvīgo paroli"),
        ("Use both passwords", "Izmantot abas paroles"),
        ("Set permanent password", "Iestatīt pastāvīgo paroli"),
        ("Enable Remote Restart", "Iespējot attālo restartēšanu"),
        ("Allow remote restart", "Atļaut attālo restartēšanu"),
        ("Restart Remote Device", "Restartēt attālo ierīci"),
        ("Are you sure you want to restart", "Vai tiešām vēlaties restartēt"),
        ("Restarting Remote Device", "Attālās ierīces restartēšana"),
        ("remote_restarting_tip", "Attālā ierīce tiek restartēta, lūdzu, aizveriet šo ziņojuma lodziņu un pēc kāda laika izveidojiet savienojumu ar pastāvīgo paroli"),
        ("Copied", "Kopēts"),
        ("Exit Fullscreen", "Iziet no pilnekrāna"),
        ("Fullscreen", "Pilnekrāna režīms"),
        ("Mobile Actions", "Mobilās darbības"),
        ("Select Monitor", "Atlasīt monitoru"),
        ("Control Actions", "Kontroles darbības"),
        ("Display Settings", "Displeja iestatījumi"),
        ("Ratio", "Attiecība"),
        ("Image Quality", "Attēla kvalitāte"),
        ("Scroll Style", "Ritināšanas stils"),
        ("Show Toolbar", "Rādīt rīkjoslu"),
        ("Hide Toolbar", "Slēpt rīkjoslu"),
        ("Direct Connection", "Tiešais savienojums"),
        ("Relay Connection", "Releja savienojums"),
        ("Secure Connection", "Drošs savienojums"),
        ("Insecure Connection", "Nedrošs savienojums"),
        ("Scale original", "Mērogs oriģināls"),
        ("Scale adaptive", "Mērogs adaptīvs"),
        ("General", "Vispārīgi"),
        ("Security", "Drošība"),
        ("Theme", "Motīvs"),
        ("Dark Theme", "Tumšais motīvs"),
        ("Light Theme", "Gaišais motīvs"),
        ("Dark", "Tumšs"),
        ("Light", "Gaišs"),
        ("Follow System", "Sekot sistēmai"),
        ("Enable hardware codec", "Iespējot aparatūras kodeku"),
        ("Unlock Security Settings", "Atbloķēt drošības iestatījumus"),
        ("Enable Audio", "Iespējot audio"),
        ("Unlock Network Settings", "Atbloķēt tīkla iestatījumus"),
        ("Server", "Serveris"),
        ("Direct IP Access", "Tiešā IP piekļuve"),
        ("Proxy", "Starpniekserveris"),
        ("Apply", "Lietot"),
        ("Disconnect all devices?", "Atvienot visas ierīces?"),
        ("Clear", "Notīrīt"),
        ("Audio Input Device", "Audio ievades ierīce"),
        ("Use IP Whitelisting", "Izmantot balto IP sarakstu"),
        ("Network", "Tīkls"),
        ("Enable RDP", "Iespējot RDP"),
        ("Pin Toolbar", "Piespraust rīkjoslu"),
        ("Unpin Toolbar", "Atspraust rīkjoslu"),
        ("Recording", "Ierakstīšana"),
        ("Directory", "Direktorija"),
        ("Automatically record incoming sessions", "Automātiski ierakstīt ienākošās sesijas"),
        ("Change", "Mainīt"),
        ("Start session recording", "Sākt sesijas ierakstīšanu"),
        ("Stop session recording", "Apturēt sesijas ierakstīšanu"),
        ("Enable Recording Session", "Iespējot sesijas ierakstīšanu"),
        ("Allow recording session", "Atļaut sesijas ierakstīšanu"),
        ("Enable LAN Discovery", "Iespējot LAN atklāšanu"),
        ("Deny LAN Discovery", "Liegt LAN atklāšanu"),
        ("Write a message", "Rakstīt ziņojumu"),
        ("Prompt", "Uzvedne"),
        ("Please wait for confirmation of UAC...", "Lūdzu, uzgaidiet UAC apstiprinājumu..."),
        ("elevated_foreground_window_tip", "Pašreizējā attālās darbvirsmas loga darbībai ir nepieciešamas lielākas tiesības, tāpēc tas īslaicīgi nevar izmantot peli un tastatūru. Varat pieprasīt attālajam lietotājam samazināt pašreizējo logu vai noklikšķināt uz pacēluma pogas savienojuma pārvaldības logā. Lai izvairītos no šīs problēmas, ieteicams instalēt programmatūru attālajā ierīcē."),
        ("Disconnected", "Atvienots"),
        ("Other", "Cits"),
        ("Confirm before closing multiple tabs", "Apstiprināt pirms vairāku cilņu aizvēršanas"),
        ("Keyboard Settings", "Tastatūras iestatījumi"),
        ("Full Access", "Pilna piekļuve"),
        ("Screen Share", "Ekrāna kopīgošana"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland nepieciešama Ubuntu 21.04 vai jaunāka versija."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland nepieciešama augstāka Linux distro versija. Lūdzu, izmēģiniet X11 desktop vai mainiet savu OS."),
        ("JumpLink", "Skatīt"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Lūdzu, atlasiet kopīgojamo ekrānu (darbojieties sesijas pusē)."),
        ("Show RustDesk", "Rādīt RustDesk"),
        ("This PC", "Šis dators"),
        ("or", "vai"),
        ("Continue with", "Turpināt ar"),
        ("Elevate", "Pacelt"),
        ("Zoom cursor", "Tālummaiņas kursors"),
        ("Accept sessions via password", "Pieņemt sesijas, izmantojot paroli"),
        ("Accept sessions via click", "Pieņemt sesijas, noklikšķinot"),
        ("Accept sessions via both", "Pieņemt sesijas, izmantojot abus"),
        ("Please wait for the remote side to accept your session request...", "Lūdzu, uzgaidiet, kamēr attālā puse pieņems jūsu sesijas pieprasījumu..."),
        ("One-time Password", "Vienreizējā parole"),
        ("Use one-time password", "Izmantot vienreizējo paroli"),
        ("One-time password length", "Vienreizējās paroles garums"),
        ("Request access to your device", "Pieprasīt piekļuvi savai ierīcei"),
        ("Hide connection management window", "Slēpt savienojuma pārvaldības logu"),
        ("hide_cm_tip", "Atļaut paslēpšanu tikai tad, ja akceptējat sesijas, izmantojot paroli un pastāvīgo paroli"),
        ("wayland_experiment_tip", "Wayland atbalsts ir eksperimentālā stadijā. Ja nepieciešama neuzraudzīta piekļuve, lūdzu, izmantojiet X11."),
        ("Right click to select tabs", "Ar peles labo pogu noklikšķiniet, lai atlasītu cilnes"),
        ("Skipped", "Izlaists"),
        ("Add to Address Book", "Pievienot adrešu grāmatai"),
        ("Group", "Grupa"),
        ("Search", "Meklēt"),
        ("Closed manually by web console", "Manuāli aizvērta tīmekļa konsole"),
        ("Local keyboard type", "Vietējais tastatūras veids"),
        ("Select local keyboard type", "Izvēlēties vietējās tastatūras veidu"),
        ("software_render_tip", "Ja izmantojat Nvidia grafikas karti operētājsistēmā Linux un attālais logs tiek aizvērts uzreiz pēc savienojuma izveides, var palīdzēt pārslēgšanās uz atvērtā koda Nouveau draiveri un izvēle izmantot programmatūras renderēšanu. Nepieciešama programmatūras restartēšana."),
        ("Always use software rendering", "Vienmēr izmantot programmatūras renderēšanu"),
        ("config_input", "Lai vadītu attālo darbvirsmu ar tastatūru, jums ir jāpiešķir RustDesk \"Ievades uzraudzība\" atļaujas."),
        ("config_microphone", "Lai runātu attālināti, jums ir jāpiešķir RustDesk \"Ierakstīt audio\" atļaujas."),
        ("request_elevation_tip", "Paaugstinājumu var pieprasīt arī tad, ja attālajā pusē ir kāds cilvēks."),
        ("Wait", "Pagaidiet"),
        ("Elevation Error", "Paaugstinājuma kļūda"),
        ("Ask the remote user for authentication", "Lūdziet attālajam lietotājam autentifikāciju"),
        ("Choose this if the remote account is administrator", "Izvēlieties šo, ja attālais konts ir administrators"),
        ("Transmit the username and password of administrator", "Pārsūtīt administratora lietotājvārdu un paroli"),
        ("still_click_uac_tip", "Joprojām attālajam lietotājam ir jānoklikšķina uz Labi UAC logā, kurā darbojas RustDesk."),
        ("Request Elevation", "Pieprasīt paaugstinājumu"),
        ("wait_accept_uac_tip", "Lūdzu, uzgaidiet, līdz attālais lietotājs pieņems UAC dialoglodziņu."),
        ("Elevate successfully", "Paaugstināšana veiksmīga"),
        ("uppercase", "lielie burti"),
        ("lowercase", "mazie burti"),
        ("digit", "cipars"),
        ("special character", "speciāla rakstzīme"),
        ("length>=8", "garums>=8"),
        ("Weak", "Vāji"),
        ("Medium", "Vidējs"),
        ("Strong", "Spēcīgs"),
        ("Switch Sides", "Pārslēgt puses"),
        ("Please confirm if you want to share your desktop?", "Lūdzu, apstipriniet, vai vēlaties koplietot savu darbvirsmu?"),
        ("Display", "Displejs"),
        ("Default View Style", "Noklusējuma skata stils"),
        ("Default Scroll Style", "Noklusējuma ritināšanas stils"),
        ("Default Image Quality", "Noklusējuma attēla kvalitāte"),
        ("Default Codec", "Noklusējuma kodeks"),
        ("Bitrate", "Bitu pārraides ātrums"),
        ("FPS", "FPS"),
        ("Auto", "Auto"),
        ("Other Default Options", "Citas noklusējuma opcijas"),
        ("Voice call", "Balss zvans"),
        ("Text chat", "Teksta tērzēšana"),
        ("Stop voice call", "Apturēt balss zvanu"),
        ("relay_hint_tip", "Iespējams, nav iespējams izveidot savienojumu tieši; varat mēģināt izveidot savienojumu, izmantojot releju. Turklāt, ja vēlaties izmantot releju pirmajā mēģinājumā, ID varat pievienot sufiksu \"/r\". vai pēdējo sesiju kartē atlasiet opciju \"Vienmēr izveidot savienojumu, izmantojot releju\", ja tāda pastāv."),
        ("Reconnect", "Atkārtoti savienot"),
        ("Codec", "Kodeks"),
        ("Resolution", "Izšķirtspēja"),
        ("No transfers in progress", "Notiek pārsūtīšana"),
        ("Set one-time password length", "Iestatīt vienreizējās paroles garumu"),
        ("install_cert_tip", "Instalēt RustDesk sertifikātu"),
        ("confirm_install_cert_tip", "Šis ir RustDesk testēšanas sertifikāts, kuram var uzticēties. Sertifikāts tiks izmantots, lai uzticētos un vajadzības gadījumā instalētu RustDesk draiverus."),
        ("RDP Settings", "RDP iestatījumi"),
        ("Sort by", "Kārtot pēc"),
        ("New Connection", "Jauns savienojums"),
        ("Restore", "Atjaun. uz leju"),
        ("Minimize", "Minimizēt"),
        ("Maximize", "Maksimizēt"),
        ("Your Device", "Jūsu ierīce"),
        ("empty_recent_tip", "Hmm, pēdējo sesiju nav!\nLaiks plānot jaunu."),
        ("empty_favorite_tip", "Vēl nav iecienītākās sesijas?\nAtradīsim kādu, ar ko sazināties, un pievienosim to jūsu izlasei!"),
        ("empty_lan_tip", "Ak nē! Šķiet, ka mēs vēl neesam atklājuši nevienu sesiju."),
        ("empty_address_book_tip", "Ak vai, izskatās, ka jūsu adrešu grāmatā šobrīd nav neviena sesija."),
        ("eg: admin", "piemēram: admin"),
        ("Empty Username", "Tukšs lietotājvārds"),
        ("Empty Password", "Tukša parole"),
        ("Me", "Es"),
        ("identical_file_tip", "Šis fails ir identisks sesijas failam."),
        ("show_monitors_tip", "Rādīt monitorus rīkjoslā"),
        ("View Mode", "Skatīšanas režīms"),
        ("login_linux_tip", "Jums ir jāpiesakās attālajā Linux kontā, lai iespējotu X darbvirsmas sesiju"),
        ("verify_rustdesk_password_tip", "Pārbaudīt RustDesk paroli"),
        ("remember_account_tip", "Atcerēties šo kontu"),
        ("os_account_desk_tip", "Šis konts tiek izmantots, lai pieteiktos attālajā operētājsistēmā un iespējotu darbvirsmas sesiju fonā"),
        ("OS Account", "OS konts"),
        ("another_user_login_title_tip", "Cits lietotājs jau ir pieteicies"),
        ("another_user_login_text_tip", "Atvienot"),
        ("xorg_not_found_title_tip", "Xorg nav atrasts"),
        ("xorg_not_found_text_tip", "Lūdzu, instalējiet Xorg"),
        ("no_desktop_title_tip", "Nav pieejama darbvirsma"),
        ("no_desktop_text_tip", "Lūdzu, instalējiet GNOME darbvirsmu"),
        ("No need to elevate", "Nav nepieciešams paaugstināt"),
        ("System Sound", "Sistēmas skaņa"),
        ("Default", "Noklusējums"),
        ("New RDP", "Jauns RDP"),
        ("Fingerprint", "Pirkstu nospiedums"),
        ("Copy Fingerprint", "Kopēt pirkstu nospiedumu"),
        ("no fingerprints", "nav pirkstu nospiedumu"),
        ("Select a peer", "Atlasīt līdzīgu"),
        ("Select peers", "Atlasīt līdzīgus"),
        ("Plugins", "Spraudņi"),
        ("Uninstall", "Atinstalēt"),
        ("Update", "Atjaunināt"),
        ("Enable", "Iespējot"),
        ("Disable", "Atspējot"),
        ("Options", "Opcijas"),
        ("resolution_original_tip", "Sākotnējā izšķirtspēja"),
        ("resolution_fit_local_tip", "Atbilst vietējai izšķirtspējai"),
        ("resolution_custom_tip", "Pielāgota izšķirtspēja"),
        ("Collapse toolbar", "Sakļaut rīkjoslu"),
        ("Accept and Elevate", "Pieņemt un paaugstināt"),
        ("accept_and_elevate_btn_tooltip", "Pieņemt savienojumu un paaugstināt UAC atļaujas."),
        ("clipboard_wait_response_timeout_tip", "Noildze gaidot atbildi uz kopiju."),
        ("Incoming connection", "Ienākošais savienojums"),
        ("Outgoing connection", "Izejošais savienojums"),
        ("Exit", "Iziet"),
        ("Open", "Atvērt"),
        ("logout_tip", "Vai tiešām vēlaties iziet?"),
        ("Service", "Serviss"),
        ("Start", "Sākt"),
        ("Stop", "Apturēt"),
        ("exceed_max_devices", "Esat sasniedzis maksimālo pārvaldāmo ierīču skaitu."),
        ("Sync with recent sessions", "Sinhronizācija ar pēdējām sesijām"),
        ("Sort tags", "Kārtot tagus"),
        ("Open connection in new tab", "Atvērt savienojumu jaunā cilnē"),
        ("Move tab to new window", "Pārvietot cilni uz jaunu logu"),
        ("Can not be empty", "Nevar būt tukšs"),
        ("Already exists", "Jau eksistē"),
        ("Change Password", "Mainīt paroli"),
        ("Refresh Password", "Atsvaidzināt paroli"),
        ("ID", "ID"),
        ("Grid View", "Režģa skats"),
        ("List View", "Saraksta skats"),
        ("Select", "Atlasīt"),
        ("Toggle Tags", "Pārslēgt tagus"),
        ("pull_ab_failed_tip", "Neizdevās atsvaidzināt adrešu grāmatu"),
        ("push_ab_failed_tip", "Neizdevās sinhronizēt adrešu grāmatu ar serveri"),
        ("synced_peer_readded_tip", "Ierīces, kas bija pēdējās sesijās, tiks sinhronizētas atpakaļ ar adrešu grāmatu."),
        ("Change Color", "Mainīt krāsu"),
        ("Primary Color", "Primārā krāsa"),
        ("HSV Color", "HSV krāsa"),
        ("Installation Successful!", "Instalēšana veiksmīga!"),
        ("Installation failed!", "Instalēšana neizdevās!"),
        ("Reverse mouse wheel", "Reversīvs peles ritenis"),
        ("{} sessions", "{} sesijas"),
        ("scam_title", "Tevi var APKRĀPT!"),
        ("scam_text1", "Ja sarunājaties ar kādu, kuru nepazīstat un kurš ir lūdzis izmantot RustDesk, lai sāktu pakalpojumu, neturpiniet un nekavējoties nolieciet klausuli."),
        ("scam_text2", "Viņi, visticamāk, ir krāpnieki, kas mēģina nozagt tavu naudu vai citu privātu informāciju."),
        ("Don't show again", "Vairs nerādīt"),
        ("I Agree", "Es piekrītu"),
        ("Decline", "Noraidīt"),
        ("Timeout in minutes", "Noildze minūtēs"),
        ("auto_disconnect_option_tip", "Automātiski aizvērt ienākošās sesijas lietotāja neaktivitātes gadījumā"),
        ("Connection failed due to inactivity", "Automātiski atvienots neaktivitātes dēļ"),
        ("Check for software update on startup", "Startējot pārbaudīt, vai nav programmatūras atjauninājumu"),
        ("upgrade_rustdesk_server_pro_to_{}_tip", "Lūdzu, jauniniet RustDesk Server Pro uz versiju {} vai jaunāku!"),
        ("pull_group_failed_tip", "Neizdevās atsvaidzināt grupu"),
        ("Filter by intersection", "")
    ].iter().cloned().collect();
}
