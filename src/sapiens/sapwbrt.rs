#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
use crate::acam::{NewPtr, SectRect};
use crate::cbridge::{FILE, blackColor, fclose, fopen, getc, patOr, srcCopy, whiteColor};
use crate::langue::Langue::GB;
use crate::port::{PortId, Ports};
use crate::sapiens::sapcolorcvt::SapColor2Index;
use crate::sapiens::sapgest::Sapgest;
use crate::sapiens::saptxt::{CvtDefaultLangRess, CvtLangRess};
use crate::sapiens::sapwind::{SapiensExit, ValideUpdate};
use crate::sapiens::{DEFAULT_PALCOUL, ECHELLE, IndexColor, Mob, NB_MAX_MOB, PalCoul};
use crate::shared::{Ptr, RGBColor, Rect, RgnHandle, SapPort};
use crate::stubs::{
    CarboniseInvalRect, CarboniseSetPatGray, CarboniseValideUpdate, ConvertPNGFileToOffscreen,
    DisposePtr, FixeCheminRelatif, GetPicture, GetRectFilePng, PopPath, PushPath, SaisitChemin,
};
use crate::{Sapiens, cbridge, port};

const TAILLE_TFR: u32 = 66;

/***********************************************************************
               SetPathInData
=> id de la picture,aire
***********************************************************************/
pub(crate) fn SetPathInData(sapgest: &Sapgest, name: *mut i8) {
    let mut vol: i16 = 0;
    let mut path: i64 = 0;

    FixeCheminRelatif(
        sapgest.volumeApplication,
        sapgest.PathApplication,
        c"data".as_ptr() as *mut i8,
    );
    SaisitChemin(&mut vol, std::ptr::null_mut(), &mut path);
    FixeCheminRelatif(vol, path, name);
}

impl Ports {
    /*************************************************************************
                    tfrima
    Vue latérale:
    Transfere le virtuel Ecran sur le reel mainWindow
    Transfert des 100 premiere ligne de Ecran vers mainWindow
    *************************************************************************/
    pub(crate) fn tfrima(&mut self) {
        let rsourc = Rect::new(0, 0, (255 * ECHELLE) as i16, (100 * ECHELLE) as i16);
        let rdest = Rect::new(
            (32 * ECHELLE) as i16,
            1,
            ((32 + 255) * ECHELLE) as i16,
            ((1 + 100) * ECHELLE) as i16,
        );
        self.CopyOffscreen(PortId::Ecran1, PortId::MainWindow, rsourc, &rdest);
        CarboniseValideUpdate();
    }

    /*************************************************************************
                    tfrdemi
    Vue latérale :
    Transfere la partie du virtuel ou se trace les personnages
    sur le reel
    *************************************************************************/
    pub(crate) fn tfrdemi(&mut self) {
        let rsourc = Rect::new(
            0,
            (32 * ECHELLE) as i16,
            (255 * ECHELLE) as i16,
            ((32 + TAILLE_TFR) * ECHELLE) as i16,
        );
        let rdest = Rect::new(
            (32 * ECHELLE) as i16,
            (33 * ECHELLE) as i16,
            ((32 + 255) * ECHELLE) as i16,
            ((33 + TAILLE_TFR) * ECHELLE) as i16,
        );
        self.CopyOffscreen(PortId::Ecran1, PortId::MainWindow, rsourc, &rdest);
    }

    /****************************************************************************
                DoZoomRect
    *****************************************************************************/
    pub(crate) fn DoZoomRect(&self, rs: &Rect) -> Rect {
        self.current().zoomed(rs)
    }

    /****************************************************************************
                DoEraseRect
    *****************************************************************************/
    pub(crate) fn DoEraseRect(&mut self, aire: &Rect) {
        let r = self.DoZoomRect(aire);
        self.EraseRect(&r);
    }

    /****************************************************************************
                DoFrameRect
    *****************************************************************************/
    pub(crate) fn DoFrameRect(&mut self, aire: &Rect) {
        let r = self.DoZoomRect(aire);
        self.FrameRect(&r);
    }
    /****************************************************************************
                DoPaintRect
    *****************************************************************************/
    pub(crate) fn DoPaintRect(&mut self, aire: &Rect) {
        let r = self.DoZoomRect(aire);
        self.PaintRect(&r);
    }
    /****************************************************************************
                DoPenSize
    *****************************************************************************/
    pub(crate) fn DoPenSize(&mut self, x: i16, y: i16) {
        let z = self.zoom();
        self.PenSize((x as f32 * z) as i16, (y as f32 * z) as i16);
    }
    /****************************************************************************
                DoMoveTo
    *****************************************************************************/
    pub(crate) fn DoMoveTo(&mut self, x: i16, y: i16) {
        let z = self.zoom();
        self.MoveTo((x as f32 * z) as i16, (y as f32 * z) as i16);
    }

    /****************************************************************************
                DoTextSize
    *****************************************************************************/
    pub(crate) fn DoTextSize(&mut self, x: i16) {
        let z = self.zoom();
        self.TextSize((x as f32 * z) as i16);
    }
    /****************************************************************************
                DoTextBox
    *****************************************************************************/
    pub(crate) fn DoTextBox(&mut self, text: &[u8], box_: &Rect, just: i16) {
        let dest = self.DoZoomRect(box_);
        self.TextBox(text, &dest, just);
    }
    /****************************************************************************
                DoScrollRect
    *****************************************************************************/
    pub(crate) fn DoScrollRect(&mut self, r: &Rect, dh: i16, dv: i16, updateRgn: RgnHandle) {
        let z = self.zoom();
        let dest = self.DoZoomRect(r);
        let bdh = (dh as f32 * z) as i16;
        let bdv = (dv as f32 * z) as i16;
        self.ScrollRect(&dest, bdh, bdv, updateRgn);
        CarboniseInvalRect(r);
        ValideUpdate();
    }
}

/******************************************************************************************
                                Read
=> Lecture d'une zone
******************************************************************************************/
fn Read(f: *mut FILE, mut p: Ptr, taille: i64) -> bool {
    let mut t = taille;
    unsafe {
        while t > 0 {
            *p = getc(f) as u8;
            p = p.add(1);
            t -= 1;
        }
    }
    true
}
/******************************************************************
                    wintel2moto
Les fichiers .BRT stockent les entiers sur 2 octets en big-endian
(Mac) : swap octets sur petit-boutiste (identite sur gros-boutiste)
******************************************************************/
fn wintel2moto(x: i16) -> i16 {
    let u = x as u16;
    (((u << 8) & 0xFF00) | ((u >> 8) & 0x00FF)) as i16
}

impl Sapiens {
    /******************************************************************
                        ChargeFichierBrt
    Chargement des fichiers MOBS en RVB 256 couleurs
    => nom du fichier, premier mob
    <= Nombre de mobs charges ou -1 si erreur
    ******************************************************************/
    pub(crate) fn ChargeFichierBrt(&mut self, nom: &str, id: i16, nbMobs: i16, vide: i16) -> i16 {
        let f: *mut FILE;
        let mut nbcol: i16 = 0;
        let mut nbligne: i16 = 0;
        let mut nb_objets: i16 = 0;
        let mut r: u8 = 0;
        let mut g: u8 = 0;
        let mut b: u8 = 0;
        unsafe {
            let chaine = cbridge::convert_to_c_char_ptr(nom);
            /* On ouvre le fichier en lecture */
            f = fopen(chaine, c"rb".as_ptr());
            if f.is_null() {
                self.Collapse(&format!("Charge fichier Brt : Fichier '{nom}' non trouve"));
                return -1;
            }
            if getc(f) != b'B' as i32
                || getc(f) != b'R' as i32
                || getc(f) != b'U' as i32
                || getc(f) != b'T' as i32
            {
                self.Collapse("Charge fichier Brt : Mauvaise en tete");
                fclose(f);
                return -1;
            }
            if self.sapwbrt.nbPalettes as i32 >= 16 {
                self.Collapse("Plus de PalCoul libre");
                fclose(f);
                return -1;
            }
            /* Nombre d'entrees dans la PalCoul */
            if !Read(f, &mut r as *mut u8 as Ptr, 1) {
                return -1;
            }
            let n = (r as i16) + 1;
            let palette = &mut self.sapwbrt.TabPalette[self.sapwbrt.nbPalettes as usize];
            palette.nbEntrees = n;
            palette.tabRgb = vec![RGBColor::default(); n as usize];
            palette.tabIndex = vec![0; n as usize];
            /* On memorise le nombre d'entrees de la PalCoul */
            for i in 0..n as usize {
                if !Read(f, &mut r as *mut u8 as Ptr, 1) {
                    return -1;
                }
                if !Read(f, &mut g as *mut u8 as Ptr, 1) {
                    return -1;
                }
                if !Read(f, &mut b as *mut u8 as Ptr, 1) {
                    return -1;
                }
                let rgb = &mut palette.tabRgb[i];
                rgb.red = ((r as u16) << 8) + r as u16;
                rgb.green = ((g as u16) << 8) + g as u16;
                rgb.blue = ((b as u16) << 8) + b as u16;
                palette.tabIndex[i] = SapColor2Index(rgb);
            }
            /* On charge les donnees de configuration */
            if !Read(f, &mut nb_objets as *mut i16 as Ptr, 2) {
                return -1;
            }
            /* wintel2moto : swap octets sur petit-boutiste */
            nb_objets = wintel2moto(nb_objets);
            if nb_objets != nbMobs {
                println!("ChargeFichierBrt err : {nom} nb_objets={nb_objets} nbMobs={nbMobs}");
                SapiensExit(-1);
            }
            /* On charge les mobs */
            let mut i = nb_objets as i32;
            let mut curid = id;
            while i > 0 {
                i -= 1;
                if !Read(f, &mut nbcol as *mut i16 as Ptr, 2) {
                    return -1;
                }
                if !Read(f, &mut nbligne as *mut i16 as Ptr, 2) {
                    return -1;
                }
                /* wintel2moto : swap octets sur petit-boutiste */
                nbcol = wintel2moto(nbcol);
                nbligne = wintel2moto(nbligne);
                let taille = (nbcol as i64) * (nbligne as i64);
                self.sapwbrt.TabMob[curid as usize].nbcol = nbcol;
                self.sapwbrt.TabMob[curid as usize].nbligne = nbligne;
                self.sapwbrt.TabMob[curid as usize].indexPalette = self.sapwbrt.nbPalettes;
                self.sapwbrt.TabMob[curid as usize].datas = NewPtr(taille);
                self.sapwbrt.TabMob[curid as usize].vide = vide;
                if self.sapwbrt.TabMob[curid as usize].datas.is_null() {
                    self.Collapse("Plus de place memoire");
                    return -1;
                }
                if !Read(f, self.sapwbrt.TabMob[curid as usize].datas, taille) {
                    return -1;
                }
                curid = (curid as i32 + 1) as i16;
                if curid as i32 >= NB_MAX_MOB as i32 {
                    self.Collapse("NB_MAX_MOB depasse");
                    return -1;
                }
            }
            /* On ferme le fichier */
            fclose(f);
        }
        self.sapwbrt.nbPalettes = (self.sapwbrt.nbPalettes as i32 + 1) as i16;
        nb_objets
    }

    /******************************************************************
                        CalculePaletteAttenuee
    Calcule une palette demi teinte
    La lut correspondant est allouee c'est a l'appellant de la liberer
    -> numero d'un mob appartenant a cette palette, coefficient d'attenuation, nombre maximum de couleurs
    a traiter
    <- ptr sur le tableau des index sur la palette (short *)
    ******************************************************************/
    pub(crate) fn CalculePaletteAttenuee(&self, num: i16, coeff: f32, nbCoul: i16) -> Vec<i16> {
        let coeffl = (256.0f32 * coeff) as i64;
        let mob = &self.sapwbrt.TabMob[num as usize];
        /* Palette associee a ce mob */
        let palette = &self.sapwbrt.TabPalette[mob.indexPalette as usize];
        /* On alloue un tableau d'index de taille identique */
        let palette_length = palette.nbEntrees.max(0) as usize;
        let mut tabIndex = Vec::with_capacity(palette_length);
        for i in 0..palette_length {
            if (i as i16) < nbCoul {
                let mut rgb = palette.tabRgb[i]; // yes copy
                let mut val = (coeffl * rgb.red as i64) >> 8;
                if val > 65535 {
                    val = 65535;
                }
                rgb.red = val as u16;
                val = (coeffl * rgb.green as i64) >> 8;
                if val > 65535 {
                    val = 65535;
                }
                rgb.green = val as u16;
                val = (coeffl * rgb.blue as i64) >> 8;
                if val > 65535 {
                    val = 65535;
                }
                rgb.blue = val as u16;
                tabIndex.push(SapColor2Index(&rgb));
            } else {
                tabIndex.push(SapColor2Index(&palette.tabRgb[i]));
            }
        }
        tabIndex
    }

    /******************************************************************
                        LibereDataMobs
    Libere la memoire de toutes les donnees des mobs et palettes
    ******************************************************************/
    pub(crate) fn LibereDataMobs(&mut self) {
        for mob in &mut self.sapwbrt.TabMob {
            if !mob.datas.is_null() {
                DisposePtr(mob.datas);
            }
            mob.datas = std::ptr::null_mut();
        }
        self.sapwbrt.nbPalettes = 0;
    }

    /******************************************************************
                        CalculeAireMob
    ******************************************************************/
    pub(crate) const fn CalculeAireMob(&self, num: i16, x: i16, y: i16) -> Rect {
        let mob = &self.sapwbrt.TabMob[num as usize];
        let x = ((x as i32) * (ECHELLE as i32)) as i16;
        let y = ((y as i32) * (ECHELLE as i32)) as i16;
        Rect::new(
            x,
            y,
            (x as i32 + mob.nbcol as i32) as i16,
            (y as i32 + mob.nbligne as i32) as i16,
        )
    }

    /*************************************************************************
                    Affmob3d
    Affiche un mob sur le port en cours
    *************************************************************************/
    pub(crate) fn Affmob3d(&mut self, num: i16, mon_aire: Rect, tabIndex: Option<&[i16]>) {
        self.sapwbrt
            .Affmob3d(self.sapwind.ports.current_mut(), num, mon_aire, tabIndex);
    }

    /******************************************************************
                        Affmobbrt
    Affiche un mob en 256 couleurs sur le port courrant
    => Numero du mob, position x (0-320),position y (0-200), sens (0-1),
    tableau des index de palette ou NULL si palette standard du Mob
    <= rien
    ******************************************************************/
    pub(crate) fn Affmobbrt(
        &mut self,
        num: i16,
        x: i16,
        y: i16,
        sens: i16,
        tabIndex: Option<&[i16]>,
    ) {
        self.sapwbrt
            .Affmobbrt(self.sapwind.ports.current_mut(), num, x, y, sens, tabIndex);
    }

    /******************************************************************
                        AffmobCar
    Affiche un mob en 256 couleurs sur le port courrant
    => Numero du mob, position x, position y, couleur
    <= rien
    ******************************************************************/
    pub(crate) fn AffmobCar(&mut self, num: i16, x: i16, y: i16, couleur: i16) {
        let mob = &self.sapwbrt.TabMob[num as usize];
        let src = mob.data();
        let mut tx: i16 = mob.nbcol;
        let mut ty: i16 = mob.nbligne;
        let mut p: usize = 0;
        let mut x = x;
        let mut y = y;
        /* Origine des donnees de l'ecran */
        let port = self.sapwind.ports.current_mut();
        let tailleLigne = port.row_bytes as isize;
        let rPort = port.bounds;
        /* Test de sortie du mob */
        /* Sortie Haut */
        if y < 0 {
            /* Entierment hors ecran */
            if (y as i32 + ty as i32) < rPort.top as i32 {
                return;
            }
            ty = (ty as i32 + y as i32) as i16;
            p += ((-(y as i32)) * tx as i32) as usize;
            y = 0;
        }
        /* Sortie basse */
        if y as i32 + ty as i32 >= rPort.bottom as i32 {
            if y >= rPort.bottom {
                return;
            }
            ty = (rPort.bottom as i32 - y as i32) as i16;
        }
        let mut delttx: i16 = 0;
        /* Sortie droite */
        if x as i32 + tx as i32 >= rPort.right as i32 {
            if x >= rPort.right {
                return;
            }
            delttx = (x as i32 + tx as i32 - rPort.right as i32) as i16;
        }
        /* Sortie gauche */
        if x < 0 {
            if x as i32 + tx as i32 <= 0 {
                return;
            }
            delttx = (-(x as i32)) as i16;
            p += delttx as usize;
            x = 0;
        }
        tx = (tx as i32 - delttx as i32) as i16;
        /* Position d'affichage sur l'ecran */
        let mut pdeb = y as isize * tailleLigne + x as isize;
        /* Couleur transparente */
        let vide: u8 = mob.vide as u8;
        /* Nombre de lignes a tracer */
        for _ in 0..ty.max(0) {
            /* Debut de  ligne */
            let mut pecr = pdeb;
            pdeb += tailleLigne;
            for _ in 0..tx.max(0) {
                /* l'index de la couleur est extrait */
                let coul = src[p];
                p += 1;
                /* Si egal a zéro alors couleur transparente */
                if coul != vide {
                    port.put(pecr, couleur as u8);
                }
                pecr += 1;
            }
            /* Donnees perdues si le mob est coupe en colonne */
            p += delttx as usize;
        }
    }

    /******************************************************************
                        Affmobbrtomb
    Affiche un mob en 256 couleurs avec son ombre
    => Numero du mob, position x, position y, decalage ombre
    @= Aire du mob
    ******************************************************************/
    pub(crate) fn Affmobbrtomb(
        &mut self,
        num: i16,
        x: i16,
        y: i16,
        deltah: i16,
        deltav: i16,
        r: &mut Rect,
    ) {
        let mob = &self.sapwbrt.TabMob[num as usize];
        let src = mob.data();
        let tx: i16 = mob.nbcol;
        let ty: i16 = mob.nbligne;
        /* Conversion des coordonnees */
        let deltah = ((deltah as i32) * (ECHELLE as i32)) as i16;
        let deltav = ((deltav as i32) * (ECHELLE as i32)) as i16;
        let x = ((x as i32) * (ECHELLE as i32)) as i16;
        let y = ((y as i32) * (ECHELLE as i32)) as i16;
        *r = Rect::new(
            x,
            y,
            (x as i32 + tx as i32 + deltah as i32) as i16,
            (y as i32 + ty as i32 + deltav as i32) as i16,
        );
        let mut p: usize = 0;
        /* Palette associee a ce mob */
        let tab_i = &self.sapwbrt.TabPalette[mob.indexPalette as usize].tabIndex;
        let tab_ombre = &self.sapwcvt.TabCvtCoulOmbre;
        /* Origine des donnees de l'ecran */
        let port = self.sapwind.ports.current_mut();
        let tailleLigne = port.row_bytes as isize;
        /* Position d'affichage sur l'ecran */
        let mut pdeb = y as isize * tailleLigne + x as isize;
        let delta_ombre = deltav as isize * tailleLigne + deltah as isize;
        /* Couleur transparente */
        let vide: u8 = mob.vide as u8;
        /* Pour l'ensemble des lignes */
        let mut yc = y as i32;
        for _ in 0..ty.max(0) {
            yc += 1;
            /* Debut de  ligne */
            let mut pecr = pdeb;
            pdeb += tailleLigne;
            for _ in 0..tx.max(0) {
                /* l'index de la couleur est extrait dans le mob */
                let coul = src[p];
                p += 1;
                /* Si egal a zéro alors couleur transparente */
                if coul != vide {
                    /* Dans l'ecran */
                    if (yc + deltav as i32) < (200 * ECHELLE as i32) {
                        /* Lecture de l'ecran pour extraire la couleur RVB */
                        if let Some(d) = port.px_mut(pecr + delta_ombre) {
                            *d = tab_ombre[*d as usize];
                        }
                    }
                    /* Conversion de l'index de couleur en index ST */
                    port.put(pecr, tab_i[coul as usize] as u8);
                }
                pecr += 1;
            }
        }
    }
}

impl Sapwbrt {
    /*************************************************************************
                    Affmob3d
    Affiche un mob sur `port`
    *************************************************************************/
    pub(crate) fn Affmob3d(
        &self,
        port: &mut SapPort,
        num: i16,
        mut mon_aire: Rect,
        tabIndex: Option<&[i16]>,
    ) {
        let mob = &self.TabMob[num as usize];
        let src = mob.data();
        let tx = mob.nbcol as i64;
        let ty = mob.nbligne as i64;
        let vide: u8 = mob.vide as u8;
        /* Palette associee a ce mob */
        let tab_i = tabIndex.unwrap_or(&self.TabPalette[mob.indexPalette as usize].tabIndex);
        /* Origine des donnees de l'ecran */
        let tailleLigne = port.row_bytes as usize;
        let rPort = port.bounds;
        /* Calcul des pas en X et en Y */
        let i = mon_aire.right as i32 - mon_aire.left as i32;
        let j = mon_aire.bottom as i32 - mon_aire.top as i32;
        /* Trop petit */
        if i <= 0 || j <= 0 {
            return;
        }
        let pas_x: i64 = (tx << 16) / (i as i64);
        let pas_y: i64 = (ty << 16) / (j as i64);
        let mut p: i64 = 0;
        /* Calcul des sorties */
        if mon_aire.left < 0 {
            let off = ((-(mon_aire.left as i32)) as f32 * (pas_x as f32) / 65536.0) as i64;
            p += off;
            mon_aire.left = 0;
        }
        if mon_aire.top < 0 {
            let off = ((-(mon_aire.top as i32)) as f32 * (pas_y as f32) / 65536.0) as i64;
            p += off * tx;
            mon_aire.top = 0;
        }
        if mon_aire.right > rPort.right {
            mon_aire.right = rPort.right;
        }
        if mon_aire.bottom > rPort.bottom {
            mon_aire.bottom = rPort.bottom;
        }
        if mon_aire.is_empty() {
            return;
        }
        let largeur = (mon_aire.right - mon_aire.left) as usize;
        let mut data = mon_aire.top as usize * tailleLigne + mon_aire.left as usize;
        let mut rest_y: i64 = 0;
        for _ in mon_aire.top..mon_aire.bottom {
            let mut pdeb = p;
            let mut rest_x: i64 = 0;
            for ecr in &mut port.pixels[data..data + largeur] {
                if let Some(&c) = usize::try_from(pdeb).ok().and_then(|k| src.get(k))
                    && c != vide
                {
                    *ecr = tab_i[c as usize] as u8;
                }
                rest_x += pas_x;
                pdeb += rest_x >> 16;
                rest_x &= 0xFFFF;
            }
            rest_y += pas_y;
            p += (rest_y >> 16) * tx;
            rest_y &= 0xFFFF;
            data += tailleLigne;
        }
    }

    /******************************************************************
                        Affmobbrt
    Affiche un mob en 256 couleurs sur `port`
    => Numero du mob, position x (0-320),position y (0-200), sens (0-1),
    tableau des index de palette ou NULL si palette standard du Mob
    <= rien
    ******************************************************************/
    pub(crate) fn Affmobbrt(
        &self,
        port: &mut SapPort,
        num: i16,
        x: i16,
        y: i16,
        sens: i16,
        tabIndex: Option<&[i16]>,
    ) {
        let mob = &self.TabMob[num as usize];
        let src = mob.data();
        let mut x = ((x as i32) * (ECHELLE as i32)) as i16;
        let mut y = ((y as i32) * (ECHELLE as i32)) as i16;
        let mut tx: i16 = mob.nbcol;
        let mut ty: i16 = mob.nbligne;
        let mut p: usize = 0;
        /* Palette associee a ce mob */
        let tab_i = tabIndex.unwrap_or(&self.TabPalette[mob.indexPalette as usize].tabIndex);
        /* Origine des donnees de l'ecran */
        let tailleLigne = port.row_bytes as isize;
        let rPort = port.bounds;
        /* Test de sortie du mob */
        /* Sortie Haut */
        if y < 0 {
            /* Entierment hors ecran */
            if (y as i32 + ty as i32) < rPort.top as i32 {
                return;
            }
            ty = (ty as i32 + y as i32) as i16;
            p += ((-(y as i32)) * tx as i32) as usize;
            y = 0;
        }
        /* Sortie basse */
        if y as i32 + ty as i32 >= rPort.bottom as i32 {
            if y >= rPort.bottom {
                return;
            }
            ty = (rPort.bottom as i32 - y as i32) as i16;
        }
        let dx: isize;
        let mut delttx: i16 = 0;
        if sens != 0 {
            dx = -1;
            /* Sortie droite */
            if x >= rPort.right {
                if x as i32 - tx as i32 >= rPort.right as i32 {
                    return;
                }
                delttx = (x as i32 - rPort.right as i32) as i16;
                x = rPort.right;
                p += delttx as usize;
            }
            /* Sortie gauche */
            if (x as i32 - tx as i32) < 0 {
                if x < 0 {
                    return;
                }
                delttx = (tx as i32 - x as i32) as i16;
            }
        } else {
            dx = 1;
            /* Sortie droite */
            if x as i32 + tx as i32 >= rPort.right as i32 {
                if x >= rPort.right {
                    return;
                }
                delttx = (x as i32 + tx as i32 - rPort.right as i32) as i16;
            }
            /* Sortie gauche */
            if x < 0 {
                if x as i32 + tx as i32 <= 0 {
                    return;
                }
                delttx = (-(x as i32)) as i16;
                p += delttx as usize;
                x = 0;
            }
        }
        tx = (tx as i32 - delttx as i32) as i16;
        /* Position d'affichage sur l'ecran */
        let mut pdeb = y as isize * tailleLigne + x as isize;
        /* Couleur transparente */
        let vide: u8 = mob.vide as u8;
        /* Nombre de lignes a tracer */
        for _ in 0..ty.max(0) {
            /* Debut de  ligne */
            let mut pecr = pdeb;
            pdeb += tailleLigne;
            for _ in 0..tx.max(0) {
                /* l'index de la couleur est extrait */
                let coul = src[p];
                p += 1;
                /* Si egal a zéro alors couleur transparente */
                if coul != vide {
                    port.put(pecr, tab_i[coul as usize] as u8);
                }
                pecr += dx;
            }
            /* Donnees perdues si le mob est coupe en colonne */
            p += delttx as usize;
        }
    }
}

impl SapPort {
    /******************************************************************
                        PlotIndexPixBrt
    Ecrit le pixel en coordonnée x, y de couleur systeme coul
    => x (0-640) ,y(0-200), index de la couleur systeme
    ******************************************************************/
    pub(crate) fn PlotIndexPixBrt(&mut self, x: i16, y: i16, coul: i16) {
        let idx = y as isize * self.row_bytes as isize + x as isize;
        self.put(idx, coul as u8);
    }

    /******************************************************************
                        GetIndexPixBrt
    Retourne l'index de la couleur dans la palette systeme
    correspondant a un point dans le port courrant
    => x,y
    <= couleur
    ******************************************************************/
    pub(crate) fn GetIndexPixBrt(&self, x: i16, y: i16) -> i16 {
        let idx = y as isize * self.row_bytes as isize + x as isize;
        self.px(idx).unwrap_or(0) as i16
    }

    /******************************************************************
                        SwapCouleur
    dans un rectangle du port en entree swappe deux couleurs
    ******************************************************************/
    pub(crate) fn SwapCouleur(&mut self, r: &Rect, coul1: i16, coul2: i16) {
        let tailleLigne = self.row_bytes as isize;
        let (c1, c2) = (coul1 as u8, coul2 as u8);
        for y in r.top..r.bottom {
            let deb = y as isize * tailleLigne + r.left as isize;
            for k in 0..(r.right - r.left).max(0) as isize {
                if let Some(d) = self.px_mut(deb + k) {
                    if *d == c1 {
                        *d = c2;
                    } else if *d == c2 {
                        *d = c1;
                    }
                }
            }
        }
    }
}

impl Sapiens {
    /****************************************************************************
                                BoxFill
    trace une boite sur le port specifie
    Les  coordonnees sont donnees en ecran ST 320x200
    (C) Ligne dep,Col dep,Ligne fin,col.fin,couleur ST
    ***************************************************************************/
    pub(crate) fn BoxFill(&mut self, port_id: PortId, r: &mut Rect, couleur: i16) {
        if r.is_empty() {
            return;
        }
        let couleur = self.sapwcvt.TabCvtCoul[couleur as usize];
        let port = self.sapwind.ports.port_mut(port_id);
        let tailleLigne = port.row_bytes as usize;
        if let Some(aire) = SectRect(r, &port.bounds) {
            let tx = (aire.right - aire.left) as usize;
            /* Position d'affichage sur l'ecran */
            let mut data = tailleLigne * aire.top as usize + aire.left as usize;
            for _ in aire.top..aire.bottom {
                port.pixels[data..data + tx].fill(couleur);
                data += tailleLigne;
            }
        }
    }
}

impl Ports {
    /******************************************************************
                        RemplitZoneLignes
                        Travaille sur le port courrant
    Remplit la zone  avec le tableau de couleur  donne
    => ligne debut (pixel ecran reel), nbre de lignes, Tableau de couleurs
    ******************************************************************/
    pub(crate) fn RemplitZoneLignes(&mut self, y: i16, tab_couleur: &[IndexColor]) {
        let port = self.current_mut();
        let tailleLigne = port.row_bytes as usize;
        let deb = tailleLigne * y as usize;
        for (ligne, tc) in port.pixels[deb..].chunks_mut(tailleLigne).zip(tab_couleur) {
            ligne.fill(tc.index as u8);
        }
    }
}

static DebDeg: [u32; 35] = [
    0x8000_0000,
    0xC000_0000,
    0xA000_0000,
    0xD000_0000,
    0xC800_0000,
    0xD800_0000,
    0xAC00_0000,
    0xD400_0000,
    0xF500_0000,
    0xEC80_0000,
    0xEC80_0000,
    0xF680_0000,
    0xF568_0000,
    0xFDD0_0000,
    0xFAB4_0000,
    0xF8D0_0000,
    0xFDBC_8000,
    0xFEEC_8000,
    0xFBA9_0000,
    0xEEEA_0000,
    0xF6B3_0000,
    0xFDAB_4000,
    0xF6B5_A000,
    0xFDD9_4A00,
    0xFB65_2400,
    0xFDE6_1800,
    0xF779_B200,
    0xFDAF_2A80,
    0xFF75_EC68,
    0xFF7D_6D94,
    0xFDD6_B6D0,
    0xFDCB_4A14,
    0xF6EC_6560,
    0xEFBA_9300,
    0xF775_B540,
];
static tabpourc: [u16; 8] = [3276, 9830, 19660, 32767, 45874, 55704, 62258, 65535];

impl Sapiens {
    /**************************************************************************
                            TraceDisque
    Tracage d'un disque
    ; => AncXD,AncXG_,AncY_: pos depart
    ;    D5-d6                 Arrivee
    ;    Epaissr_               Epaisseur
    ;
    ***************************************************************************/
    pub(crate) fn TraceDisque(
        &mut self,
        dg: &mut i16,
        dd: &mut i16,
        dy: &mut i16,
        fg: i16,
        fd: i16,
        epaisseur: i16,
    ) {
        let mut fy: i32 = 0;
        if *dy > 0 && epaisseur > 0 {
            let palhaute = self.sapglob.palhaute.borrow();
            let tcole = &self.sapwarb.tabcoul;
            let cvt = &self.sapwcvt.TabCvtCoul;
            let port = self.sapwind.ports.current_mut();
            let tailleLigne = port.row_bytes as isize;
            fy = *dy as i32 * ECHELLE as i32;
            let mut ep = epaisseur as i32 * ECHELLE as i32;
            let mut x1: i64 = ((*dg as i64) << 16) * (ECHELLE as i64);
            let mut x2: i64 = ((*dd as i64) << 16) * (ECHELLE as i64);
            let dx1: i64 = ((ECHELLE as i64) * ((fg as i64 - *dg as i64) << 16)) / (ep as i64);
            let dx2: i64 = ((ECHELLE as i64) * ((fd as i64 - *dd as i64) << 16)) / (ep as i64);
            while fy != 0 && ep != 0 {
                ep -= 1;
                if fy < 200 {
                    let delta = (x2 - x1) >> 16;
                    let mut sx1: i16 = (x1 >> 16) as i16;
                    for i in 0..8 {
                        let index1 = cvt[palhaute[i] as usize];
                        let index2 = cvt[palhaute[tcole[i + 1] as usize] as usize];
                        let sx2: i16 = ((x1 >> 16) + ((delta * tabpourc[i] as i64) >> 16)) as i16;
                        let mut sx1c = sx1;
                        let mut sx2c = sx2;
                        if sx1c < 0 {
                            sx1c = 0;
                        }
                        if sx2c > 511 {
                            sx2c = 511;
                        }
                        /* On a sx1 sx2 debut et fin en colonne du morceau de  ligne */
                        if sx1c < 512 && sx2c > 0 && sx1c != sx2c {
                            /* Pointeur sur l'ecran */
                            let p = tailleLigne * fy as isize + sx1c as isize;
                            let tsx = sx2c - sx1c;
                            let degrade = DebDeg[((if tsx > 32 { 31i16 } else { tsx - 1 })
                                + (fy as i16 & 3))
                                as usize];
                            let mut dg2 = degrade;
                            for k in 0..tsx.max(0) as isize {
                                let c = if dg2 & 0x8000_0000 != 0 {
                                    index1
                                } else {
                                    index2
                                };
                                port.put(p + k, c);
                                dg2 <<= 1;
                            }
                        }
                        sx1 = sx2c;
                    }
                }
                x1 += dx1;
                x2 += dx2;
                fy -= 1;
            }
        }
        *dg = fg;
        *dd = fd;
        if fy <= 0 {
            *dy = 0;
        } else {
            *dy = (fy / (ECHELLE as i32)) as i16;
        }
    }
}
/**************************************************************************
                                                SapiensCreeOffscreen
Cree un offscreen de taille donnee, compatible avec le MaxDevice
=> fenetre d'affichage, taille X offscreen, taille Y offscreen
<= pointeur sur CGrafPort ou NULL si erreur
**************************************************************************/
pub(crate) fn SapiensCreeOffscreen(taillex: i16, tailley: i16) -> SapPort {
    port::create(
        Rect {
            top: 0,
            left: 0,
            bottom: tailley,
            right: taillex,
        },
        8,
    )
}

impl Ports {
    /**************************************************************************
                            CopyOffscreen
    Copy un rectangle entre la source et la destination
    **************************************************************************/
    pub(crate) fn CopyOffscreen(
        &mut self,
        source: PortId,
        dest: PortId,
        rsourceIn: Rect,
        rdestIn: &Rect,
    ) {
        if rsourceIn.is_empty() {
            return;
        }
        /* Chaque port porte son échelle : un rectangle est mis à l'échelle
        uniquement s'il est adressé à la fenêtre. */
        let realSource = self.port(source).zoomed(&rsourceIn);
        let realDest = self.port(dest).zoomed(rdestIn);
        // Sur ecran
        if self.is_window(dest) {
            let _g = self.on(dest);
            self.BackColor(whiteColor as i16);
            self.ForeColor(blackColor as i16);
            self.CopyBits(source, dest, &realSource, &realDest, srcCopy as i16);
        } else {
            /* Copie entre deux offscreen avec palette identique */
            let boundSource = self.port(source).bounds;
            let mut boundDest = self.port(dest).bounds;
            let tailleLigneSource = self.port(source).row_bytes as usize;
            let tailleLigneDest = self.port(dest).row_bytes as usize;
            /* Par defaut, le rectangle a transferer est la source */
            let rtfrs = realSource;
            /* On passe le bound destination dans le repere de la source */
            boundDest.OffsetRect(
                realSource.left - realDest.left,
                realSource.top - realDest.top,
            );
            /* On intersecte les 3 rectangles */
            let Some(rtfrs) = SectRect(&rtfrs, &boundSource).and_then(|r| SectRect(&r, &boundDest))
            else {
                return;
            };
            /* On recalcule le rectangle destination */
            let mut rtfrd = rtfrs;
            rtfrd.OffsetRect(
                realDest.left - realSource.left,
                realDest.top - realSource.top,
            );
            let mut dataSource = tailleLigneSource * rtfrs.top as usize + rtfrs.left as usize;
            let mut dataDest = tailleLigneDest * rtfrd.top as usize + rtfrd.left as usize;
            let tx = (rtfrs.right - rtfrs.left) as usize;
            /* La source peut être la destination : on passe par un tampon. */
            let mut ligne = vec![0u8; tx];
            for _ in rtfrs.top..rtfrs.bottom {
                ligne.copy_from_slice(&self.port(source).pixels[dataSource..dataSource + tx]);
                self.port_mut(dest).pixels[dataDest..dataDest + tx].copy_from_slice(&ligne);
                dataSource += tailleLigneSource;
                dataDest += tailleLigneDest;
            }
        }
    }
}

impl Sapiens {
    /*************************************************************************
                    effdemi
    Efface a partir du virtuel ou se trace les personnages en
    recopiant la premiere partie de l'ecran virtuel sur la deuxieme
    *************************************************************************/
    pub(crate) fn effdemi(&mut self) {
        let rsourc = Rect::new(
            0,
            (32 * ECHELLE) as i16,
            (255 * ECHELLE) as i16,
            ((32 + TAILLE_TFR) * ECHELLE) as i16,
        );
        self.sapwind
            .ports
            .CopyOffscreen(PortId::Ecran2, PortId::Ecran1, rsourc, &rsourc);
    }

    /*************************************************************************
                    Effima :
    Efface l'ecran virtuel Ecran en entier
    => couleur ST d'effacement
    *************************************************************************/
    pub(crate) fn effima(&mut self, couleur: i16) {
        let mut r = Rect::new(0, 0, (255 * ECHELLE) as i16, (100 * ECHELLE) as i16);
        self.BoxFill(PortId::Ecran1, &mut r, couleur);
    }

    /*************************************************************************
                    effvirt
    Efface la zone de la vue 3d sur Debecr
    => Couleur RVB St
    *************************************************************************/
    pub(crate) fn effvirt(&mut self, couleur: i16) {
        let mut r = Rect::new(
            (32 * ECHELLE) as i16,
            ECHELLE as i16,
            ((32 + 255) * ECHELLE) as i16,
            (101 * ECHELLE) as i16,
        );
        self.BoxFill(PortId::Debecr, &mut r, couleur);
    }

    /*************************************************************************
                    tfrvirt
    Transfere l'aire de la vue 3d de l'ecran de travail vers l'ecran
    Debecr virtuel
    Transfere d'Ecran (premiere partie) vers Debecr
    *************************************************************************/
    pub(crate) fn tfrvirt(&mut self) {
        let rsourc = Rect::new(0, 0, (255 * ECHELLE) as i16, (100 * ECHELLE) as i16);
        let rdest = Rect::new(
            (32 * ECHELLE) as i16,
            ECHELLE as i16,
            ((32 + 255) * ECHELLE) as i16,
            ((1 + 100) * ECHELLE) as i16,
        );
        self.sapwind
            .ports
            .CopyOffscreen(PortId::Ecran1, PortId::Debecr, rsourc, &rdest);
    }

    /*************************************************************************
                    tfrmenufenetre
    => actif, inactif
    *************************************************************************/
    pub(crate) fn tfrmenufenetre(&mut self, etat: i16) {
        let r = Rect::new(
            0,
            (113 * ECHELLE) as i16,
            (320 * ECHELLE) as i16,
            (200 * ECHELLE) as i16,
        );
        if etat != 0 {
            self.sapwind
                .ports
                .CopyOffscreen(PortId::Debecr, PortId::MainWindow, r, &r);
        } else {
            let _g = self.sapwind.ports.on(PortId::MainWindow);
            CarboniseSetPatGray();
            let rgb = RGBColor {
                red: 15000,
                green: 6000,
                blue: 2000,
            };

            self.sapwind.ports.RGBForeColor(&rgb);
            self.sapwind.ports.PenMode(patOr as i16);
            self.sapwind.ports.DoPaintRect(&r);
            self.sapwind.ports.ForeColor(blackColor as i16);
            self.sapwind.ports.PenNormal();
        }
    }
    /*************************************************************************
                    CopyEcran1Sur2
    *************************************************************************/
    pub(crate) fn CopyEcran1Sur2(&mut self) {
        let rsourc = Rect::new(0, 0, (255 * ECHELLE) as i16, (100 * ECHELLE) as i16);
        self.sapwind
            .ports
            .CopyOffscreen(PortId::Ecran1, PortId::Ecran2, rsourc, &rsourc);
    }
    /*************************************************************************
                    TfrEcr
    Transfere une aire de Debecr vers la fenetre visible
    Transfere de Debecr vers MainWindow
    *************************************************************************/
    pub(crate) fn TfrEcr(&mut self, r: Option<Rect>) {
        if let Some(r) = &r {
            self.sapwind
                .ports
                .CopyOffscreen(PortId::Debecr, PortId::MainWindow, *r, r);
        } else {
            let r2 = Rect::new(0, 0, (320 * ECHELLE) as i16, (200 * ECHELLE) as i16);
            self.sapwind
                .ports
                .CopyOffscreen(PortId::Debecr, PortId::MainWindow, r2, &r2);
        }
    }
    /*************************************************************************
                    Affmob
    Affiche un mob sur l'ecran virtuel
    => Numero du mob, position y (0-200) ,position x (0-320), sens
    *************************************************************************/
    pub(crate) fn Affmob(&mut self, mob: i16, y: i16, x: i16, inverse: i16) {
        let _g = self.sapwind.ports.on(PortId::Ecran1);
        self.Affmobbrt(mob, x, y, inverse, None);
    }
    /*************************************************************************
                    AffmobConst
    Affiche un mob sur l'ecran virtuel constant et l'ecran virtuel volatile
    => Numero du mob, position y (0-200) ,position x (0-320), sens
    *************************************************************************/
    pub(crate) fn AffmobConst(&mut self, mob: i16, y: i16, x: i16, inverse: i16) {
        let _g = self.sapwind.ports.on(PortId::Ecran1);
        self.Affmobbrt(mob, x, y, inverse, None);
        self.sapwind.ports.set(PortId::Ecran2);
        self.Affmobbrt(mob, x, y, inverse, None);
    }
    /*************************************************************************
                    AffDebecr
    Affiche un mob sur l'ecran Debecr
    => Numero du mob, position y (0-200) ,position x (0-320), transfert
    *************************************************************************/
    pub(crate) fn AffDebecr(&mut self, mob: i16, y: i16, x: i16, transfere: i16) {
        let _g = self.sapwind.ports.on(PortId::Debecr);
        self.Affmobbrt(mob, x, y, 0, None);
        if transfere != 0 {
            let r = self.CalculeAireMob(mob, x, y);
            self.TfrEcr(Some(r));
        }
    }
    /*************************************************************************
                    affomb
    Affiche un mob sur l'ecran de travail (Debecr) et le recopie sur le reel
    => Coordonnees en 320x200
    *************************************************************************/
    pub(crate) fn affomb(
        &mut self,
        mob: i16,
        ligne: i16,
        colonne: i16,
        deltav: i16,
        deltah: i16,
        recopie: bool,
    ) {
        let mut r = Rect::default();

        let _g = self.sapwind.ports.on(PortId::Debecr);
        self.Affmobbrtomb(mob, colonne, ligne, deltah, deltav, &mut r);
        if recopie {
            self.sapwind
                .ports
                .CopyOffscreen(PortId::Debecr, PortId::MainWindow, r, &r);
        }
    }

    /*************************************************************************
                    AffPalette
    Affiche la palette en cours
    *************************************************************************/
    pub(crate) fn AffPalette(&mut self) {
        let _g = self.sapwind.ports.on(PortId::Debecr);
        let port = self.sapwind.ports.current_mut();
        let tailleLigne = port.row_bytes as usize;
        const TAILLE_CARRE: usize = 16;
        for i in 0..16usize {
            for j in 0..16usize {
                let coul = (i * 16 + j) as u8;
                let mut p = i * TAILLE_CARRE * tailleLigne + j * TAILLE_CARRE + 32;
                for _k in 0..TAILLE_CARRE {
                    port.pixels[p..p + TAILLE_CARRE].fill(coul);
                    p += tailleLigne;
                }
            }
        }
        self.TfrEcr(None);
    }

    /***********************************************************************
                   AffichePictDansAire
    => id de la picture,aire
    ***********************************************************************/
    pub(crate) fn AffichePictDansAire(&mut self, num: i16, r: &Rect) {
        let rDest = self.sapwind.ports.DoZoomRect(r);
        PushPath();
        /* recherche de la picture en fichier */
        let nom = format!("image_{}.png", CvtDefaultLangRess(num),);
        SetPathInData(&self.sapgest, c"images".as_ptr() as *mut i8);
        let mut rpng = Rect::default();
        if GetRectFilePng(&nom, &mut rpng) {
            let offscreen = ConvertPNGFileToOffscreen(&nom);
            if let Some(offscreen) = offscreen {
                self.sapwind.ports.BackColor(whiteColor as i16);
                self.sapwind.ports.ForeColor(blackColor as i16);
                self.sapwind.ports.CopyBits(
                    offscreen,
                    self.sapwind.ports.current_id(),
                    &rpng,
                    &rDest,
                    srcCopy as i16,
                );
            }
        } else {
            let mut pict = GetPicture(CvtDefaultLangRess(num));
            if pict.is_null() {
                pict = GetPicture(CvtLangRess(GB as i16, num));
            }
            if !pict.is_null() {
                self.sapwind.ports.DrawPicture(pict, &rDest);
            }
        }
        PopPath();
    }
}

pub(crate) struct Sapwbrt {
    TabPalette: [PalCoul; 16],
    nbPalettes: i16,
    pub(crate) TabMob: [Mob; 640usize],
}

impl Default for Sapwbrt {
    fn default() -> Self {
        Self {
            TabPalette: [DEFAULT_PALCOUL; 16],
            nbPalettes: 0,
            TabMob: unsafe { std::mem::zeroed() },
        }
    }
}
