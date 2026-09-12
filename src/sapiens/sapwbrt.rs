#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
use crate::acam::{
    AcamClosePortOffscreen, AcamCreatePortOffscreen, AcamGetInfoOffscreen, AcamGetPortOffscreen,
    AcamSetPortOffscreen, BackColor, BlockMove, CarboniseGetCurrentPort,
    CarboniseGetPortBitMapForCopyBits, CarboniseGetWindowPort, CarboniseSetPortWindow, CopyBits,
    DisposPtr, EmptyRect, EraseRect, ForeColor, FrameRect, Line, LineTo, MoveTo, NewPtr,
    OffsetRect, PaintRect, PenMode, PenNormal, PenSize, RGBForeColor, ScrollRect, SectRect,
    SetRect,
};
use crate::cbridge::{
    DialogPtr, FILE, FixeChemin, GetMainDevice, RetraceRect, blackColor, fclose, fopen, fscanf,
    getc, patOr, printf, sprintf, srcCopy, strcpy, whiteColor,
};
use crate::langue::Langue::GB;
use crate::sapiens::sapcolorcvt::SapColor2Index;
use crate::sapiens::sapglob::palhaute;
use crate::sapiens::saptxt::{CvtDefaultLangRess, CvtLangRess};
use crate::sapiens::sapwarb::tabcoul;
use crate::sapiens::sapwcvt::{TabCvtCoul, TabCvtCoulOmbre};
use crate::sapiens::sapwind::{
    Collapse, Ecran1, Ecran2, SapZoom, SapiensExit, ValideUpdate, debecr, mainWindow,
};
use crate::sapiens::{ECHELLE, IndexColorPtr, Mob, NB_MAX_MOB, PalCoul};
use crate::shared::RGBColor as BRGBColor;
use crate::shared::{CGrafPtr, Handle, PicHandle, Ptr, RGBColor, Rect, RgnHandle};
use crate::stubs::{
    CarboniseInvalRect, CarboniseSetPatGray, CarboniseValideUpdate, ConvertPNGFileToOffscreen,
    DisposePtr, DrawPicture, FixeCheminRelatif, GetDItem, GetPicture, GetRectFilePng,
    PathApplication, PopPath, PushPath, SaisitChemin, volumeApplication,
};
use crate::text::{TextBox, TextSize};
use std::mem::size_of;
use std::os::raw::{c_char, c_int, c_long, c_short, c_void};

const TAILLE_TFR: u32 = 66;

/*********************************************************************
                    GetRectMainEcran
Donne l'aire de l'ecran principal
 ********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn GetRectMainEcran(aireEcran: *mut Rect) {
    unsafe {
        let gd = GetMainDevice();
        *aireEcran = (*(*gd)).gdRect;
    }
}

/*********************************************************************
                            RetraceItem
Retrace instanatanemen,t l'item demand�
********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn RetraceItem(dialog: DialogPtr, item: ::std::os::raw::c_short) {
    let mut itemType: c_short = 0;
    let mut handle = Handle::default();
    let mut box_ = Rect::default();

    unsafe {
        GetDItem(dialog, item, &mut itemType, &mut handle, &mut box_);
        RetraceRect(dialog, &mut box_);
    }
}
/***********************************************************************
               SetPathInData
=> id de la picture,aire
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn SetPathInData(name: *mut ::std::os::raw::c_char) {
    let mut vol: c_short = 0;
    let mut path: c_long = 0;

    unsafe {
        FixeCheminRelatif(
            volumeApplication,
            PathApplication,
            c"data".as_ptr() as *mut c_char,
        );
        SaisitChemin(&mut vol, std::ptr::null_mut(), &mut path);
        FixeCheminRelatif(vol, path, name);
    }
}

/*************************************************************************
                tfrima
Vue latérale:
Transfere le virtuel Ecran sur le reel mainWindow
Transfert des 100 premiere ligne de Ecran vers mainWindow
*************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn tfrima() {
    let mut rsourc = Rect::default();
    let mut rdest = Rect::default();
    unsafe {
        SetRect(
            &mut rsourc,
            (0 * ECHELLE) as i16,
            (0 * ECHELLE) as i16,
            (255 * ECHELLE) as i16,
            (100 * ECHELLE) as i16,
        );
        SetRect(
            &mut rdest,
            (32 * ECHELLE) as i16,
            (1 * ECHELLE) as i16,
            ((32 + 255) * ECHELLE) as i16,
            ((1 + 100) * ECHELLE) as i16,
        );
        CopyOffscreen(
            Ecran1,
            CarboniseGetWindowPort(mainWindow),
            &mut rsourc,
            &mut rdest,
        );
        CarboniseValideUpdate();
    }
}

/*************************************************************************
                tfrdemi
Vue latérale :
Transfere la partie du virtuel ou se trace les personnages
sur le reel
*************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn tfrdemi() {
    let mut rsourc = Rect::default();
    let mut rdest = Rect::default();
    unsafe {
        SetRect(
            &mut rsourc,
            (0 * ECHELLE) as i16,
            (32 * ECHELLE) as i16,
            (255 * ECHELLE) as i16,
            ((32 + TAILLE_TFR) * ECHELLE) as i16,
        );
        SetRect(
            &mut rdest,
            (32 * ECHELLE) as i16,
            (33 * ECHELLE) as i16,
            ((32 + 255) * ECHELLE) as i16,
            ((33 + TAILLE_TFR) * ECHELLE) as i16,
        );
        CopyOffscreen(
            Ecran1,
            CarboniseGetWindowPort(mainWindow),
            &mut rsourc,
            &mut rdest,
        );
    }
}

/****************************************************************************
            DoZoomRect
*****************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn DoZoomRect(rs: *mut Rect, realDest: *mut Rect) {
    unsafe {
        SetRect(
            realDest,
            (((*rs).left as f32) * SapZoom) as i16,
            (((*rs).top as f32) * SapZoom) as i16,
            (((*rs).right as f32) * SapZoom) as i16,
            (((*rs).bottom as f32) * SapZoom) as i16,
        );
    }
}

/****************************************************************************
            DoEraseRect
*****************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn DoEraseRect(aire: *mut Rect) {
    let mut r = Rect::default();

    DoZoomRectSiEcranReel(aire, &mut r);
    EraseRect(&r);
}

/****************************************************************************
            DoZoomRectSiEcranReel
*****************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn DoZoomRectSiEcranReel(rs: *mut Rect, realDest: *mut Rect) {
    unsafe {
        if CarboniseGetCurrentPort() == CarboniseGetWindowPort(mainWindow) {
            DoZoomRect(rs, realDest);
        } else {
            *realDest = *rs;
        }
    }
}

/****************************************************************************
            DoFrameRect
*****************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn DoFrameRect(aire: *mut Rect) {
    let mut r = Rect::default();
    DoZoomRectSiEcranReel(aire, &mut r);
    FrameRect(&mut r);
}
/****************************************************************************
            DoPaintRect
*****************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn DoPaintRect(aire: *mut Rect) {
    let mut r = Rect::default();
    DoZoomRectSiEcranReel(aire, &mut r);
    PaintRect(&mut r);
}
/****************************************************************************
            DoPenSize
*****************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn DoPenSize(x: c_short, y: c_short) {
    unsafe {
        if CarboniseGetCurrentPort() == CarboniseGetWindowPort(mainWindow) {
            PenSize((x as f32 * SapZoom) as i16, (y as f32 * SapZoom) as i16);
        } else {
            PenSize(x, y);
        }
    }
}
/****************************************************************************
            DoMoveTo
*****************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn DoMoveTo(x: c_short, y: c_short) {
    unsafe {
        if CarboniseGetCurrentPort() == CarboniseGetWindowPort(mainWindow) {
            MoveTo((x as f32 * SapZoom) as i16, (y as f32 * SapZoom) as i16);
        } else {
            MoveTo(x, y);
        }
    }
}
/****************************************************************************
            DoLineTo
*****************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn DoLineTo(x: c_short, y: c_short) {
    unsafe {
        if CarboniseGetCurrentPort() == CarboniseGetWindowPort(mainWindow) {
            LineTo((x as f32 * SapZoom) as i16, (y as f32 * SapZoom) as i16);
        } else {
            LineTo(x, y);
        }
    }
}
/****************************************************************************
            DoLine
*****************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn DoLine(x: c_short, y: c_short) {
    unsafe {
        if CarboniseGetCurrentPort() == CarboniseGetWindowPort(mainWindow) {
            Line((x as f32 * SapZoom) as i16, (y as f32 * SapZoom) as i16);
        } else {
            Line(x, y);
        }
    }
}
/****************************************************************************
            DoTextSize
*****************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn DoTextSize(x: c_short) {
    unsafe {
        if CarboniseGetCurrentPort() == CarboniseGetWindowPort(mainWindow) {
            TextSize((x as f32 * SapZoom) as i16);
        } else {
            TextSize(x);
        }
    }
}
/****************************************************************************
            DoTextBox
*****************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn DoTextBox(text: *mut c_char, length: c_short, box_: *mut Rect, just: c_short) {
    let mut dest = Rect::default();
    unsafe {
        DoZoomRectSiEcranReel(box_, &mut dest);
        TextBox(text as *const c_void, length as i64, &dest, just);
    }
}
/****************************************************************************
            DoScrollRect
*****************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn DoScrollRect(r: *mut Rect, dh: c_short, dv: c_short, updateRgn: RgnHandle) {
    let mut dest = Rect::default();
    unsafe {
        DoZoomRectSiEcranReel(r, &mut dest);
        let mut bdh = dh;
        let mut bdv = dv;
        if CarboniseGetCurrentPort() == CarboniseGetWindowPort(mainWindow) {
            bdh = (dh as f32 * SapZoom) as i16;
            bdv = (dv as f32 * SapZoom) as i16;
        }
        ScrollRect(&dest, bdh, bdv, updateRgn);
        CarboniseInvalRect(r);
        ValideUpdate();
    }
}
/******************************************************************************************
                                Read
=> Lecture d'une zone
******************************************************************************************/
fn Read(f: *mut FILE, mut p: Ptr, taille: i64) -> bool {
    unsafe {
        let mut t = taille;
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
fn wintel2moto(x: c_short) -> c_short {
    let u = x as u16;
    (((u << 8) & 0xFF00) | ((u >> 8) & 0x00FF)) as c_short
}
/******************************************************************
                    CvtPathMacToPC
Convertit un chemin mac en chemin PC
(identite sur Mac : le bloc #ifdef PC n'est pas compile)
******************************************************************/
#[unsafe(no_mangle)]
#[allow(unused_variables)]
pub extern "C" fn CvtPathMacToPC(nom: *mut c_char) {}
/******************************************************************
                   CvtColorToIndex
******************************************************************/
fn CvtColorToIndex(pal: *mut PalCoul, rgb: *const BRGBColor) -> c_short {
    unsafe {
        let mut deltamin: i64 = -1;
        let mut imin: c_short = 0;
        for i in 0..(*pal).nbEntrees.max(0) as usize {
            let e = (*pal).tabRgb.add(i).read();
            let dr = ((*rgb).red as i64 - e.red as i64).abs();
            let dg = ((*rgb).green as i64 - e.green as i64).abs();
            let db = ((*rgb).blue as i64 - e.blue as i64).abs();
            let delta = dr + dg + db;
            if deltamin < 0 || delta < deltamin {
                deltamin = delta;
                imin = i as c_short;
            }
        }
        imin
    }
}
/******************************************************************
                    FuturChargeFichierBrt
Chargement en PNG des anciens fichiers MOBS
******************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn FuturChargeFichierBrt(
    nom: *mut c_char,
    id: c_short,
    nbMobs: c_short,
    vide: c_short,
) -> c_short {
    unsafe {
        let mut gbak: CGrafPtr = std::ptr::null_mut();
        let mut nomPng = [0i8; 512];
        let mut offscreenSource: CGrafPtr = std::ptr::null_mut();
        let mut aire = Rect::default();
        let mut rowByte: c_short = 0;
        let mut ps: Ptr = std::ptr::null_mut();
        let mut pd: Ptr = std::ptr::null_mut();
        let mut rgb = BRGBColor::default();
        let mut f: *mut FILE = std::ptr::null_mut();

        AcamGetPortOffscreen(&mut gbak);
        PushPath();
        FixeChemin(0i64, c"PAL".as_ptr() as *mut c_char);
        sprintf(nomPng.as_mut_ptr(), c"%s.pal".as_ptr(), nom);
        f = fopen(nomPng.as_mut_ptr(), c"r".as_ptr());
        if !f.is_null() {
            let mut n: c_short = 0;
            fscanf(f, c"%hd".as_ptr(), &mut n as *mut c_short);
            TabPalette[nbPalettes as usize].nbEntrees = n;
            let taille = (n as i64) * (size_of::<BRGBColor>() as i64);
            TabPalette[nbPalettes as usize].tabRgb = NewPtr(taille) as *mut BRGBColor;
            TabPalette[nbPalettes as usize].tabIndex =
                NewPtr((n as i64) * (size_of::<c_short>() as i64)) as *mut c_short;
            for i in 0..n.max(0) as usize {
                let tri = TabPalette[nbPalettes as usize].tabIndex;
                let trr = TabPalette[nbPalettes as usize].tabRgb;
                fscanf(
                    f,
                    c" %hd %hd %hd %hd".as_ptr(),
                    tri.add(i),
                    trr.add(i) as *mut c_short,
                    (trr.add(i) as *mut c_short).add(1),
                    (trr.add(i) as *mut c_short).add(2),
                );
            }
            fclose(f);
        }
        PopPath();

        PushPath();
        FixeChemin(0i64, c"PNG".as_ptr() as *mut c_char);
        for curid in id as i32..(id as i32 + nbMobs as i32) {
            sprintf(
                nomPng.as_mut_ptr(),
                c"%s-%d.png".as_ptr(),
                nom,
                curid as c_int,
            );
            offscreenSource = ConvertPNGFileToOffscreen(nomPng.as_mut_ptr());
            if !offscreenSource.is_null() {
                ps = AcamGetInfoOffscreen(
                    offscreenSource,
                    &mut aire,
                    &mut rowByte,
                    std::ptr::null_mut(),
                ) as Ptr;
                let taille = (aire.right as i64) * (aire.bottom as i64);
                TabMob[curid as usize].nbcol = aire.right;
                TabMob[curid as usize].nbligne = aire.bottom;
                TabMob[curid as usize].indexPalette = nbPalettes;
                pd = NewPtr(taille);
                TabMob[curid as usize].datas = pd;
                TabMob[curid as usize].vide = vide;
                for y in 0..aire.bottom.max(0) as usize {
                    let mut p = ps.add((y as i64 * rowByte as i64) as usize);
                    for _ in 0..aire.right.max(0) as usize {
                        rgb.red = (*p as u16) << 8;
                        p = p.add(1);
                        rgb.green = (*p as u16) << 8;
                        p = p.add(1);
                        rgb.blue = (*p as u16) << 8;
                        p = p.add(2);
                        *pd = CvtColorToIndex(
                            std::ptr::addr_of!(TabPalette[nbPalettes as usize]) as *mut PalCoul,
                            &rgb,
                        ) as u8;
                        pd = pd.add(1);
                    }
                }
                AcamClosePortOffscreen(offscreenSource);
            }
        }
        PopPath();
        nbPalettes = (nbPalettes as i32 + 1) as i16;
        nbMobs
    }
}
/******************************************************************
                    ChargeFichierBrt
Chargement des fichiers MOBS en RVB 256 couleurs
=> nom du fichier, premier mob
<= Nombre de mobs charges ou -1 si erreur
******************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn ChargeFichierBrt(
    nom: *mut c_char,
    id: c_short,
    nbMobs: c_short,
    vide: c_short,
) -> c_short {
    unsafe {
        let mut f: *mut FILE = std::ptr::null_mut();
        let mut n: c_short = 0;
        let mut nbcol: c_short = 0;
        let mut nbligne: c_short = 0;
        let mut nb_objets: c_short = 0;
        let mut r: u8 = 0;
        let mut g: u8 = 0;
        let mut b: u8 = 0;
        let mut rgb: *mut BRGBColor = std::ptr::null_mut();
        let mut chaine = [0i8; 255];

        strcpy(chaine.as_mut_ptr(), nom);
        CvtPathMacToPC(chaine.as_mut_ptr());
        /* On ouvre le fichier en lecture */
        f = fopen(chaine.as_mut_ptr(), c"rb".as_ptr());
        if f.is_null() {
            sprintf(
                chaine.as_mut_ptr(),
                c"Charge fichier Brt : Fichier '%s' non trouv\xe9".as_ptr(),
                nom,
            );
            Collapse(chaine.as_mut_ptr());
            return -1;
        }
        if getc(f) != b'B' as c_int
            || getc(f) != b'R' as c_int
            || getc(f) != b'U' as c_int
            || getc(f) != b'T' as c_int
        {
            Collapse(c"Charge fichier Brt : Mauvaise en tete".as_ptr() as *mut c_char);
            fclose(f);
            return -1;
        }
        if nbPalettes as i32 >= 16 {
            Collapse(c"Plus de PalCoul libre".as_ptr() as *mut c_char);
            fclose(f);
            return -1;
        }
        /* Nombre d'entrees dans la PalCoul */
        if !Read(f, &mut r as *mut u8 as Ptr, 1) {
            return -1;
        }
        n = (r as c_short) + 1;
        TabPalette[nbPalettes as usize].nbEntrees = n;
        let taille = (n as i64) * (size_of::<BRGBColor>() as i64);
        rgb = NewPtr(taille) as *mut BRGBColor;
        TabPalette[nbPalettes as usize].tabRgb = rgb;
        TabPalette[nbPalettes as usize].tabIndex =
            NewPtr((n as i64) * (size_of::<c_short>() as i64)) as *mut c_short;
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
            (*rgb).red = ((r as u16) << 8) + r as u16;
            (*rgb).green = ((g as u16) << 8) + g as u16;
            (*rgb).blue = ((b as u16) << 8) + b as u16;
            (*TabPalette[nbPalettes as usize].tabIndex.add(i)) = SapColor2Index(rgb);
            rgb = rgb.add(1);
        }
        /* On charge les donnees de configuration */
        if !Read(f, &mut nb_objets as *mut c_short as Ptr, 2) {
            return -1;
        }
        /* wintel2moto : swap octets sur petit-boutiste */
        nb_objets = wintel2moto(nb_objets);
        if nb_objets != nbMobs {
            printf(
                c"ChargeFichierBrt err : %s nb_objets=%d nbMobs=%d\n".as_ptr(),
                nom,
                nb_objets as c_int,
                nbMobs as c_int,
            );
            SapiensExit(-1);
        }
        /* On charge les mobs */
        let mut i = nb_objets as i32;
        let mut curid = id;
        while i > 0 {
            i -= 1;
            if !Read(f, &mut nbcol as *mut c_short as Ptr, 2) {
                return -1;
            }
            if !Read(f, &mut nbligne as *mut c_short as Ptr, 2) {
                return -1;
            }
            /* wintel2moto : swap octets sur petit-boutiste */
            nbcol = wintel2moto(nbcol);
            nbligne = wintel2moto(nbligne);
            let taille = (nbcol as i64) * (nbligne as i64);
            TabMob[curid as usize].nbcol = nbcol;
            TabMob[curid as usize].nbligne = nbligne;
            TabMob[curid as usize].indexPalette = nbPalettes;
            TabMob[curid as usize].datas = NewPtr(taille);
            TabMob[curid as usize].vide = vide;
            if TabMob[curid as usize].datas.is_null() {
                Collapse(c"Plus de place memoire".as_ptr() as *mut c_char);
                return -1;
            }
            if !Read(f, TabMob[curid as usize].datas, taille) {
                return -1;
            }
            curid = (curid as i32 + 1) as i16;
            if curid as i32 >= NB_MAX_MOB as i32 {
                Collapse(c"NB_MAX_MOB depasse".as_ptr() as *mut c_char);
                return -1;
            }
        }
        /* On ferme le fichier */
        fclose(f);
        nbPalettes = (nbPalettes as i32 + 1) as i16;
        nb_objets
    }
}
/******************************************************************
                    CalculePaletteAttenuee
Calcule une palette demi teinte
La lut correspondant est allouee c'est a l'appellant de la liberer
-> numero d'un mob appartenant a cette palette, coefficient d'attenuation, nombre maximum de couleurs
a traiter
<- ptr sur le tableau des index sur la palette (short *)
******************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn CalculePaletteAttenuee(
    num: c_short,
    coeff: f32,
    nbCoul: c_short,
) -> *mut c_short {
    unsafe {
        let coeffl = (256.0f32 * coeff) as i64;
        let mob = &TabMob[num as usize];
        /* Palette associee a ce mob */
        let palette = &TabPalette[mob.indexPalette as usize];
        /* On alloue un tableau d'index de taille identique */
        let tabIndex = NewPtr((size_of::<c_short>() as i64) * (palette.nbEntrees.max(0) as i64))
            as *mut c_short;
        if !tabIndex.is_null() {
            for i in 0..palette.nbEntrees.max(0) as usize {
                if (i as i16) < nbCoul {
                    let mut rgb = palette.tabRgb.add(i).read();
                    let mut val = ((coeffl * rgb.red as i64) >> 8) as i64;
                    if val > 65535 {
                        val = 65535;
                    }
                    rgb.red = val as u16;
                    val = ((coeffl * rgb.green as i64) >> 8) as i64;
                    if val > 65535 {
                        val = 65535;
                    }
                    rgb.green = val as u16;
                    val = ((coeffl * rgb.blue as i64) >> 8) as i64;
                    if val > 65535 {
                        val = 65535;
                    }
                    rgb.blue = val as u16;
                    (*tabIndex.add(i)) = SapColor2Index(&mut rgb);
                } else {
                    (*tabIndex.add(i)) = SapColor2Index(palette.tabRgb.add(i));
                }
            }
        }
        tabIndex
    }
}
/******************************************************************
                    LibereDataMobs
Libere la memoire de toutes les donnees des mobs et palettes
******************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn LibereDataMobs() {
    unsafe {
        for i in 0..NB_MAX_MOB as usize {
            if !TabMob[i].datas.is_null() {
                DisposePtr(TabMob[i].datas);
            }
            TabMob[i].datas = std::ptr::null_mut();
        }
        for i in 0..nbPalettes.max(0) as usize {
            DisposPtr(TabPalette[i].tabRgb as Ptr);
            DisposPtr(TabPalette[i].tabIndex as Ptr);
        }
        nbPalettes = 0;
    }
}
/******************************************************************
                    CalculeAireMob
******************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn CalculeAireMob(num: c_short, x: c_short, y: c_short, r: *mut Rect) {
    unsafe {
        let mob = &TabMob[num as usize];
        let x = ((x as i32) * (ECHELLE as i32)) as i16;
        let y = ((y as i32) * (ECHELLE as i32)) as i16;
        SetRect(
            r,
            x,
            y,
            (x as i32 + mob.nbcol as i32) as i16,
            (y as i32 + mob.nbligne as i32) as i16,
        );
    }
}
/*************************************************************************
                Affmob3d
Affiche un mob sur le port en cours
*************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn Affmob3d(num: c_short, aire: *mut Rect, tabIndex: *mut c_short) {
    unsafe {
        let mob = &TabMob[num as usize];
        let mut mon_aire: Rect = *aire;
        let tx: c_short = mob.nbcol;
        let ty: c_short = mob.nbligne;
        let vide: u8 = mob.vide as u8;
        let mut p: *mut u8 = mob.datas;
        /* Palette associee a ce mob */
        let mut tab_i: *mut c_short = tabIndex;
        if tab_i.is_null() {
            tab_i = TabPalette[mob.indexPalette as usize].tabIndex;
        }
        /* Origine des donnees de l'ecran */
        let mut rPort = Rect::default();
        let mut tailleLigne: c_short = 0;
        let mut depth: c_short = 0;
        let mut data = AcamGetInfoOffscreen(
            CarboniseGetCurrentPort(),
            &mut rPort,
            &mut tailleLigne,
            &mut depth,
        ) as *mut u8;
        if data.is_null() {
            return;
        }
        /* Calcul des pas en X et en Y */
        let mut i = mon_aire.right as i32 - mon_aire.left as i32;
        let mut j = mon_aire.bottom as i32 - mon_aire.top as i32;
        /* Trop petit */
        if i <= 0 || j <= 0 {
            return;
        }
        let pas_x: i64 = ((tx as i64) << 16) / (i as i64);
        let pas_y: i64 = ((ty as i64) << 16) / (j as i64);
        /* Calcul des sorties */
        if mon_aire.left < 0 {
            let off = ((-(mon_aire.left as i32)) as f32 * (pas_x as f32) / 65536.0) as i64;
            p = p.add(off as usize);
            mon_aire.left = 0;
        }
        if mon_aire.top < 0 {
            let off = ((-(mon_aire.top as i32)) as f32 * (pas_y as f32) / 65536.0) as i64;
            p = p.add((off * tx as i64) as usize);
            mon_aire.top = 0;
        }
        if mon_aire.right > rPort.right {
            mon_aire.right = rPort.right;
        }
        if mon_aire.bottom > rPort.bottom {
            mon_aire.bottom = rPort.bottom;
        }
        if EmptyRect(&mon_aire) {
            return;
        }
        data = data.add((mon_aire.top as i64 * tailleLigne as i64 + mon_aire.left as i64) as usize);
        let mut rest_y: i64 = 0;
        i = mon_aire.bottom as i32 - mon_aire.top as i32 + 1;
        while {
            i -= 1;
            i != 0
        } {
            let mut ecr_deb = data;
            let mut pdeb = p;
            let mut rest_x: i64 = 0;
            j = mon_aire.right as i32 - mon_aire.left as i32 + 1;
            while {
                j -= 1;
                j != 0
            } {
                if *pdeb != vide {
                    *ecr_deb = *tab_i.add(*pdeb as usize) as u8;
                }
                ecr_deb = ecr_deb.add(1);
                rest_x += pas_x;
                pdeb = pdeb.add((rest_x >> 16) as usize);
                rest_x &= 0xFFFF;
            }
            rest_y += pas_y;
            p = p.add(((rest_y >> 16) * tx as i64) as usize);
            rest_y &= 0xFFFF;
            data = data.add(tailleLigne as usize);
        }
    }
}
/******************************************************************
                    Affmobbrt
Affiche un mob en 256 couleurs sur le port courrant
=> Numero du mob, position x (0-320),position y (0-200), sens (0-1),
tableau des index de palette ou NULL si palette standard du Mob
<= rien
******************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn Affmobbrt(
    num: c_short,
    x: c_short,
    y: c_short,
    sens: c_short,
    tabIndex: *mut c_short,
) {
    unsafe {
        let mob = &TabMob[num as usize];
        let mut x = ((x as i32) * (ECHELLE as i32)) as i16;
        let mut y = ((y as i32) * (ECHELLE as i32)) as i16;
        let mut tx: c_short = mob.nbcol;
        let mut ty: c_short = mob.nbligne;
        let mut p: *mut u8 = mob.datas;
        /* Palette associee a ce mob */
        let mut tab_i: *mut c_short = tabIndex;
        if tab_i.is_null() {
            tab_i = TabPalette[mob.indexPalette as usize].tabIndex;
        }
        /* Origine des donnees de l'ecran */
        let mut rPort = Rect::default();
        let mut tailleLigne: c_short = 0;
        let mut depth: c_short = 0;
        let data = AcamGetInfoOffscreen(
            CarboniseGetCurrentPort(),
            &mut rPort,
            &mut tailleLigne,
            &mut depth,
        ) as *mut u8;
        if data.is_null() {
            return;
        }
        /* Test de sortie du mob */
        /* Sortie Haut */
        if y < 0 {
            /* Entierment hors ecran */
            if (y as i32 + ty as i32) < rPort.top as i32 {
                return;
            }
            ty = (ty as i32 + y as i32) as i16;
            p = p.add(((-(y as i32)) * tx as i32) as usize);
            y = 0;
        }
        /* Sortie basse */
        if y as i32 + ty as i32 >= rPort.bottom as i32 {
            if y >= rPort.bottom {
                return;
            }
            ty = (rPort.bottom as i32 - y as i32) as i16;
        }
        let dx: i16;
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
                p = p.add(delttx as usize);
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
                p = p.add(delttx as usize);
                x = 0;
            }
        }
        tx = (tx as i32 - delttx as i32) as i16;
        /* Position d'affichage sur l'ecran */
        let mut pdeb = data.add((y as i64 * tailleLigne as i64 + x as i64) as usize);
        /* Couleur transparente */
        let vide: u8 = mob.vide as u8;
        /* Nombre de lignes a tracer */
        let mut j = ty as i32 + 1;
        while {
            j -= 1;
            j != 0
        } {
            /* Debut de  ligne */
            let mut pecr = pdeb;
            pdeb = pdeb.add(tailleLigne as usize);
            let mut i = tx as i32 + 1;
            while {
                i -= 1;
                i != 0
            } {
                /* l'index de la couleur est extrait */
                let coul = *p;
                p = p.add(1);
                /* Si egal a zéro alors couleur transparente */
                if coul != vide {
                    *pecr = *tab_i.add(coul as usize) as u8;
                }
                pecr = pecr.offset(dx as isize);
            }
            /* Donnees perdues si le mob est coupe en colonne */
            p = p.add(delttx as usize);
        }
    }
}
/******************************************************************
                    AffmobCar
Affiche un mob en 256 couleurs sur le port courrant
=> Numero du mob, position x, position y, couleur
<= rien
******************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn AffmobCar(num: c_short, x: c_short, y: c_short, couleur: c_short) {
    unsafe {
        let mob = &TabMob[num as usize];
        let mut tx: c_short = mob.nbcol;
        let mut ty: c_short = mob.nbligne;
        let mut p: *mut u8 = mob.datas;
        let mut x = x;
        let mut y = y;
        /* Origine des donnees de l'ecran */
        let mut rPort = Rect::default();
        let mut tailleLigne: c_short = 0;
        let mut depth: c_short = 0;
        let data = AcamGetInfoOffscreen(
            CarboniseGetCurrentPort(),
            &mut rPort,
            &mut tailleLigne,
            &mut depth,
        ) as *mut u8;
        if data.is_null() {
            return;
        }
        /* Test de sortie du mob */
        /* Sortie Haut */
        if y < 0 {
            /* Entierment hors ecran */
            if (y as i32 + ty as i32) < rPort.top as i32 {
                return;
            }
            ty = (ty as i32 + y as i32) as i16;
            p = p.add(((-(y as i32)) * tx as i32) as usize);
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
            p = p.add(delttx as usize);
            x = 0;
        }
        tx = (tx as i32 - delttx as i32) as i16;
        /* Position d'affichage sur l'ecran */
        let mut pdeb = data.add((y as i64 * tailleLigne as i64 + x as i64) as usize);
        /* Couleur transparente */
        let vide: u8 = mob.vide as u8;
        /* Nombre de lignes a tracer */
        let mut j = ty as i32 + 1;
        while {
            j -= 1;
            j != 0
        } {
            /* Debut de  ligne */
            let mut pecr = pdeb;
            pdeb = pdeb.add(tailleLigne as usize);
            let mut i = tx as i32 + 1;
            while {
                i -= 1;
                i != 0
            } {
                /* l'index de la couleur est extrait */
                let coul = *p;
                p = p.add(1);
                /* Si egal a zéro alors couleur transparente */
                if coul != vide {
                    *pecr = couleur as u8;
                }
                pecr = pecr.add(1);
            }
            /* Donnees perdues si le mob est coupe en colonne */
            p = p.add(delttx as usize);
        }
    }
}
/******************************************************************
                    Affmobbrtomb
Affiche un mob en 256 couleurs avec son ombre
=> Numero du mob, position x, position y, decalage ombre
@= Aire du mob
******************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn Affmobbrtomb(
    num: c_short,
    x: c_short,
    y: c_short,
    deltah: c_short,
    deltav: c_short,
    r: *mut Rect,
) {
    unsafe {
        let mob = &TabMob[num as usize];
        let tx: c_short = mob.nbcol;
        let ty: c_short = mob.nbligne;
        /* Conversion des coordonnees */
        let deltah = ((deltah as i32) * (ECHELLE as i32)) as i16;
        let deltav = ((deltav as i32) * (ECHELLE as i32)) as i16;
        let x = ((x as i32) * (ECHELLE as i32)) as i16;
        let y = ((y as i32) * (ECHELLE as i32)) as i16;
        SetRect(
            r,
            x,
            y,
            (x as i32 + tx as i32 + deltah as i32) as i16,
            (y as i32 + ty as i32 + deltav as i32) as i16,
        );
        let mut p: *mut u8 = mob.datas;
        /* Palette associee a ce mob */
        let tab_i: *mut c_short = TabPalette[mob.indexPalette as usize].tabIndex;
        /* Origine des donnees de l'ecran */
        let mut rPort = Rect::default();
        let mut tailleLigne: c_short = 0;
        let mut depth: c_short = 0;
        let data = AcamGetInfoOffscreen(
            CarboniseGetCurrentPort(),
            &mut rPort,
            &mut tailleLigne,
            &mut depth,
        ) as *mut u8;
        if data.is_null() {
            return;
        }
        /* Position d'affichage sur l'ecran */
        let mut pdeb = data.add((y as i64 * tailleLigne as i64 + x as i64) as usize);
        /* Couleur transparente */
        let vide: u8 = mob.vide as u8;
        /* Pour l'ensemble des lignes */
        let mut j = ty as i32;
        let mut yc = y as i32;
        while j != 0 {
            j -= 1;
            yc += 1;
            /* Debut de  ligne */
            let mut pecr = pdeb;
            let mut pomb = pdeb.add((deltav as i64 * tailleLigne as i64 + deltah as i64) as usize);
            pdeb = pdeb.add(tailleLigne as usize);
            let mut i = 0i32;
            while {
                let cur = i;
                i += 1;
                cur < tx as i32
            } {
                /* l'index de la couleur est extrait dans le mob */
                let coul = *p;
                p = p.add(1);
                /* Si egal a zéro alors couleur transparente */
                if coul != vide {
                    /* Dans l'ecran */
                    if (yc + deltav as i32) < (200 * ECHELLE as i32) {
                        /* Lecture de l'ecran pour extraire la couleur RVB */
                        *pomb = TabCvtCoulOmbre[*pomb as usize];
                    }
                    /* Conversion de l'index de couleur en index ST */
                    *pecr = *tab_i.add(coul as usize) as u8;
                }
                pecr = pecr.add(1);
                pomb = pomb.add(1);
            }
        }
    }
}
/******************************************************************
                    PlotIndexPixBrt
Ecrit le pixel en coordonnée x, y de couleur systeme coul
=> x (0-640) ,y(0-200), index de la couleur systeme
******************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn PlotIndexPixBrt(port: CGrafPtr, x: c_short, y: c_short, coul: c_short) {
    unsafe {
        let mut tailleLigne: c_short = 0;
        let mut depth: c_short = 0;
        let data = AcamGetInfoOffscreen(port, std::ptr::null_mut(), &mut tailleLigne, &mut depth)
            as *mut u8;
        if data.is_null() {
            return;
        }
        /* Position d'affichage sur l'ecran */
        let p = data.add((y as i64 * tailleLigne as i64 + x as i64) as usize);
        *p = coul as u8;
    }
}
/******************************************************************
                    GetIndexPixBrt
Retourne l'index de la couleur dans la palette systeme
correspondant a un point dans le port courrant
=> x,y
<= couleur
******************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn GetIndexPixBrt(port: CGrafPtr, x: c_short, y: c_short) -> c_short {
    unsafe {
        let mut tailleLigne: c_short = 0;
        let mut depth: c_short = 0;
        let data = AcamGetInfoOffscreen(port, std::ptr::null_mut(), &mut tailleLigne, &mut depth)
            as *mut u8;
        if data.is_null() {
            return -1;
        }
        /* Position d'affichage sur l'ecran */
        let p = data.add((y as i64 * tailleLigne as i64 + x as i64) as usize);
        *p as c_short
    }
}
/******************************************************************
                    SwapCouleur
dans un rectangle du port en entree swappe deux couleurs
******************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn SwapCouleur(port: CGrafPtr, r: *mut Rect, coul1: c_short, coul2: c_short) {
    unsafe {
        let mut tailleLigne: c_short = 0;
        let mut depth: c_short = 0;
        let data = AcamGetInfoOffscreen(port, std::ptr::null_mut(), &mut tailleLigne, &mut depth)
            as *mut u8;
        if data.is_null() {
            return;
        }
        let mut y = (*r).top;
        while y < (*r).bottom {
            let mut p = data.add((y as i64 * tailleLigne as i64 + (*r).left as i64) as usize);
            let mut x = (*r).right - (*r).left;
            while x != 0 {
                x -= 1;
                if *p == coul1 as u8 {
                    *p = coul2 as u8;
                } else if *p == coul2 as u8 {
                    *p = coul1 as u8;
                }
                p = p.add(1);
            }
            y += 1;
        }
    }
}
/****************************************************************************
                            BoxFill
trace une boite sur le port specifie
Les  coordonnees sont donnees en ecran ST 320x200
(C) Ligne dep,Col dep,Ligne fin,col.fin,couleur ST
***************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn BoxFill(port: CGrafPtr, r: *mut Rect, couleur: c_short) {
    unsafe {
        if EmptyRect(r) {
            return;
        }
        let mut aire = Rect::default();
        let mut tailleLigne: c_short = 0;
        let mut depth: c_short = 0;
        let mut data =
            AcamGetInfoOffscreen(port, &mut aire, &mut tailleLigne, &mut depth) as *mut u8;
        if data.is_null() {
            return;
        }
        if SectRect(r, &aire, &mut aire) {
            let couleur = TabCvtCoul[couleur as usize];
            let mut ty = aire.bottom as i32 - aire.top as i32;
            let tx = aire.right as i32 - aire.left as i32;
            /* Position d'affichage sur l'ecran */
            data = data.add((tailleLigne as i64 * aire.top as i64 + aire.left as i64) as usize);
            while ty != 0 {
                ty -= 1;
                std::ptr::write_bytes(data, couleur, tx as usize);
                data = data.add(tailleLigne as usize);
            }
        }
    }
}
/******************************************************************
                    RemplitLigneBrut
Remplit la zone comprise entre deux lignes avec la couleur donnee
=> ligne debut, ligne fin ,couleur
******************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn RemplitLigneBrut(y1: c_short, y2: c_short, index: c_short) {
    unsafe {
        let mut tailleLigne: c_short = 0;
        let mut depth: c_short = 0;
        let mut data = AcamGetInfoOffscreen(
            CarboniseGetCurrentPort(),
            std::ptr::null_mut(),
            &mut tailleLigne,
            &mut depth,
        ) as *mut u8;
        if data.is_null() {
            return;
        }
        let y1pix = y1 as i32 * ECHELLE as i32;
        data = data.add((tailleLigne as i64 * y1pix as i64) as usize);
        let len = ((y2 as i32 - y1 as i32) as i64) * (ECHELLE as i64) * (tailleLigne as i64);
        std::ptr::write_bytes(data, index as u8, len as usize);
    }
}
/******************************************************************
                    RemplitZoneLignes
                    Travaille sur le port courrant
Remplit la zone  avec le tableau de couleur  donne
=> ligne debut (pixel ecran reel), nbre de lignes, Tableau de couleurs
******************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn RemplitZoneLignes(y: c_short, taille: c_short, mut tab_couleur: IndexColorPtr) {
    unsafe {
        let mut tailleLigne: c_short = 0;
        let mut depth: c_short = 0;
        let mut data = AcamGetInfoOffscreen(
            CarboniseGetCurrentPort(),
            std::ptr::null_mut(),
            &mut tailleLigne,
            &mut depth,
        ) as *mut u8;
        if data.is_null() {
            return;
        }
        data = data.add((tailleLigne as i64 * y as i64) as usize);
        let mut t = taille;
        while t != 0 {
            t -= 1;
            std::ptr::write_bytes(data, (*tab_couleur).index as u8, tailleLigne as usize);
            data = data.add(tailleLigne as usize);
            tab_couleur = tab_couleur.add(1);
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

/**************************************************************************
                        TraceDisque
Tracage d'un disque
; => AncXD,AncXG_,AncY_: pos depart
;    D5-d6                 Arrivee
;    Epaissr_               Epaisseur
;
***************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn TraceDisque(
    dg: *mut c_short,
    dd: *mut c_short,
    dy: *mut c_short,
    fg: c_short,
    fd: c_short,
    epaisseur: c_short,
) {
    unsafe {
        {
            let mut fy: i32 = 0;
            if *dy > 0 && epaisseur > 0 {
                let mut tailleLigne: c_short = 0;
                let mut depth: c_short = 0;
                let data = AcamGetInfoOffscreen(
                    CarboniseGetCurrentPort(),
                    std::ptr::null_mut(),
                    &mut tailleLigne,
                    &mut depth,
                ) as *mut u8;
                if data.is_null() {
                    return;
                }
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
                        let tcole = std::ptr::addr_of!(tabcoul) as *const c_short;
                        for i in 0..8 {
                            let index1 =
                                TabCvtCoul[(*palhaute.add(*tcole.add(i) as usize)) as usize];
                            let index2 =
                                TabCvtCoul[(*palhaute.add(*tcole.add(i + 1) as usize)) as usize];
                            let sx2: i16 =
                                ((x1 >> 16) + ((delta * tabpourc[i] as i64) >> 16)) as i16;
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
                                let mut p = data
                                    .add((tailleLigne as i64 * fy as i64 + sx1c as i64) as usize);
                                let tsx = sx2c - sx1c;
                                let degrade = DebDeg[((if tsx > 32 { 31i16 } else { tsx - 1 })
                                    + (fy as i16 & 3))
                                    as usize];
                                let mut dg2 = degrade;
                                let mut ts = tsx as i64;
                                while ts != 0 {
                                    ts -= 1;
                                    if dg2 & 0x8000_0000 != 0 {
                                        *p = index1;
                                    } else {
                                        *p = index2;
                                    }
                                    p = p.add(1);
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
                *dy = (fy / (ECHELLE as i32)) as c_short;
            }
        }
    }
}
/**************************************************************************
                                                SapiensCreeOffscreen
Cree un offscreen de taille donnee, compatible avec le MaxDevice
=> fenetre d'affichage, taille X offscreen, taille Y offscreen
<= pointeur sur CGrafPort ou NULL si erreur
**************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn SapiensCreeOffscreen(taillex: c_short, tailley: c_short) -> CGrafPtr {
    AcamCreatePortOffscreen(taillex, tailley, 8)
}
/**************************************************************************
                       LibereOffscreen
Libere l'offscreen crée
**************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn LibereOffscreen(myGrafPort: CGrafPtr) {
    AcamClosePortOffscreen(myGrafPort);
}
/**************************************************************************
                        CopyOffscreen
Copy un rectangle entre la source et la destination
**************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn CopyOffscreen(
    source: CGrafPtr,
    dest: CGrafPtr,
    rsourceIn: *const Rect,
    rdestIn: *const Rect,
) {
    unsafe {
        if EmptyRect(rsourceIn) {
            return;
        }
        let mut realSource: Rect = std::mem::zeroed();
        if source == CarboniseGetWindowPort(mainWindow) {
            DoZoomRect(rsourceIn as *mut Rect, &mut realSource);
        } else {
            realSource = *rsourceIn;
        }
        let mut realDest: Rect = std::mem::zeroed();
        if dest == CarboniseGetWindowPort(mainWindow) {
            DoZoomRect(rdestIn as *mut Rect, &mut realDest);
        } else {
            realDest = *rdestIn;
        }
        // Sur ecran
        if dest == CarboniseGetWindowPort(mainWindow) {
            let mut gbak: CGrafPtr = std::ptr::null_mut();
            AcamGetPortOffscreen(&mut gbak);
            AcamSetPortOffscreen(dest);
            BackColor(whiteColor as i16);
            ForeColor(blackColor as i16);
            CopyBits(
                CarboniseGetPortBitMapForCopyBits(source),
                CarboniseGetPortBitMapForCopyBits(dest),
                &realSource,
                &realDest,
                srcCopy as i16,
                std::ptr::null_mut(),
            );
            AcamSetPortOffscreen(gbak);
        } else {
            /* Copie entre deux offscreen avec palette identique */
            let mut boundSource = Rect::default();
            let mut boundDest = Rect::default();
            let mut tailleLigneSource: c_short = 0;
            let mut tailleLigneDest: c_short = 0;
            let mut depthSource: c_short = 0;
            let mut depthDest: c_short = 0;
            let mut dataSource = AcamGetInfoOffscreen(
                source,
                &mut boundSource,
                &mut tailleLigneSource,
                &mut depthSource,
            ) as *mut u8;
            let mut dataDest =
                AcamGetInfoOffscreen(dest, &mut boundDest, &mut tailleLigneDest, &mut depthDest)
                    as *mut u8;
            /* Par defaut, le rectangle a transferer est la source */
            let mut rtfrs = realSource;
            /* On passe le bound destination dans le repere de la source */
            OffsetRect(
                &mut boundDest,
                realSource.left - realDest.left,
                realSource.top - realDest.top,
            );
            /* On intersecte les 3 rectangles */
            let rtfrs_p = &mut rtfrs as *mut Rect;
            SectRect(rtfrs_p, &boundSource, rtfrs_p);
            SectRect(rtfrs_p, &boundDest, rtfrs_p);
            /* On recalcule le rectangle destination */
            let mut rtfrd = rtfrs;
            OffsetRect(
                &mut rtfrd,
                realDest.left - realSource.left,
                realDest.top - realSource.top,
            );
            dataSource = dataSource
                .add((tailleLigneSource as i64 * rtfrs.top as i64 + rtfrs.left as i64) as usize);
            dataDest = dataDest
                .add((tailleLigneDest as i64 * rtfrd.top as i64 + rtfrd.left as i64) as usize);
            let mut ty = rtfrs.bottom as i32 - rtfrs.top as i32;
            let tx = rtfrs.right as i32 - rtfrs.left as i32;
            while ty != 0 {
                ty -= 1;
                BlockMove(
                    dataSource as *const c_void,
                    dataDest as *mut c_void,
                    tx as i64,
                );
                dataSource = dataSource.add(tailleLigneSource as usize);
                dataDest = dataDest.add(tailleLigneDest as usize);
            }
        }
    }
}
/*************************************************************************
                effdemi
Efface a partir du virtuel ou se trace les personnages en
recopiant la premiere partie de l'ecran virtuel sur la deuxieme
*************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn effdemi() {
    unsafe {
        let mut rsourc = Rect::default();
        SetRect(
            &mut rsourc,
            0,
            (32 * ECHELLE) as i16,
            (255 * ECHELLE) as i16,
            ((32 + TAILLE_TFR) * ECHELLE) as i16,
        );
        let rp = &rsourc as *const Rect;
        CopyOffscreen(Ecran2, Ecran1, rp, rp);
    }
}
/*************************************************************************
                Effima :
Efface l'ecran virtuel Ecran en entier
=> couleur ST d'effacement
*************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn effima(couleur: c_short) {
    let mut r = Rect::default();
    unsafe {
        SetRect(&mut r, 0, 0, (255 * ECHELLE) as i16, (100 * ECHELLE) as i16);
        BoxFill(Ecran1, &mut r, couleur);
    }
}
/*************************************************************************
                effvirt
Efface la zone de la vue 3d sur debecr
=> Couleur RVB St
*************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn effvirt(couleur: c_short) {
    let mut r = Rect::default();
    unsafe {
        SetRect(
            &mut r,
            (32 * ECHELLE) as i16,
            (1 * ECHELLE) as i16,
            ((32 + 255) * ECHELLE) as i16,
            (101 * ECHELLE) as i16,
        );
        BoxFill(debecr, &mut r, couleur);
    }
}
/*************************************************************************
                tfrvirt
Transfere l'aire de la vue 3d de l'ecran de travail vers l'ecran
debecr virtuel
Transfere d'Ecran (premiere partie) vers debecr
*************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn tfrvirt() {
    unsafe {
        let mut rsourc = Rect::default();
        let mut rdest = Rect::default();
        SetRect(
            &mut rsourc,
            0,
            0,
            (255 * ECHELLE) as i16,
            (100 * ECHELLE) as i16,
        );
        SetRect(
            &mut rdest,
            (32 * ECHELLE) as i16,
            (1 * ECHELLE) as i16,
            ((32 + 255) * ECHELLE) as i16,
            ((1 + 100) * ECHELLE) as i16,
        );
        CopyOffscreen(Ecran1, debecr, &rsourc, &rdest);
    }
}
/*************************************************************************
                tfrmenufenetre
=> actif, inactif
*************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn tfrmenufenetre(etat: c_short) {
    unsafe {
        let mut r = Rect::default();
        let mut gbak: CGrafPtr = std::ptr::null_mut();
        let mut rgb = RGBColor::default();

        AcamGetPortOffscreen(&mut gbak);

        SetRect(
            &mut r,
            0,
            (113 * ECHELLE) as i16,
            (320 * ECHELLE) as i16,
            (200 * ECHELLE) as i16,
        );
        if etat != 0 {
            let rp = &r as *const Rect;
            CopyOffscreen(debecr, CarboniseGetWindowPort(mainWindow), rp, rp);
        } else {
            CarboniseSetPortWindow(mainWindow);
            CarboniseSetPatGray();
            rgb.red = 15000;
            rgb.green = 6000;
            rgb.blue = 2000;

            RGBForeColor(&rgb);
            PenMode(patOr as i16);
            DoPaintRect(&mut r);
            ForeColor(blackColor as i16);
            PenNormal();
            AcamSetPortOffscreen(gbak);
        }
    }
}
/*************************************************************************
                CopyEcran1Sur2
*************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn CopyEcran1Sur2() {
    unsafe {
        let mut rsourc = Rect::default();
        SetRect(
            &mut rsourc,
            0,
            0,
            (255 * ECHELLE) as i16,
            (100 * ECHELLE) as i16,
        );
        let rp = &rsourc as *const Rect;
        CopyOffscreen(Ecran1, Ecran2, rp, rp);
    }
}
/*************************************************************************
                TfrEcr
Transfere une aire de debecr vers la fenetre visible
Transfere de debecr vers MainWindow
*************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn TfrEcr(r: *mut Rect) {
    unsafe {
        if r.is_null() {
            let mut r2 = Rect::default();
            SetRect(
                &mut r2,
                0,
                0,
                (320 * ECHELLE) as i16,
                (200 * ECHELLE) as i16,
            );
            let rp = &r2 as *const Rect;
            CopyOffscreen(debecr, CarboniseGetWindowPort(mainWindow), rp, rp);
        } else {
            CopyOffscreen(debecr, CarboniseGetWindowPort(mainWindow), r, r);
        }
    }
}
/*************************************************************************
                Affmob
Affiche un mob sur l'ecran virtuel
=> Numero du mob, position y (0-200) ,position x (0-320), sens
*************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn Affmob(mob: c_short, y: c_short, x: c_short, inverse: c_short) {
    unsafe {
        let mut gbak: CGrafPtr = std::ptr::null_mut();
        AcamGetPortOffscreen(&mut gbak);
        AcamSetPortOffscreen(Ecran1);
        Affmobbrt(mob, x, y, inverse, std::ptr::null_mut());
        AcamSetPortOffscreen(gbak);
    }
}
/*************************************************************************
                AffmobConst
Affiche un mob sur l'ecran virtuel constant et l'ecran virtuel volatile
=> Numero du mob, position y (0-200) ,position x (0-320), sens
*************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn AffmobConst(mob: c_short, y: c_short, x: c_short, inverse: c_short) {
    unsafe {
        let mut gbak: CGrafPtr = std::ptr::null_mut();
        AcamGetPortOffscreen(&mut gbak);
        AcamSetPortOffscreen(Ecran1);
        Affmobbrt(mob, x, y, inverse, std::ptr::null_mut());
        AcamSetPortOffscreen(Ecran2);
        Affmobbrt(mob, x, y, inverse, std::ptr::null_mut());
        AcamSetPortOffscreen(gbak);
    }
}
/*************************************************************************
                AffDebecr
Affiche un mob sur l'ecran debecr
=> Numero du mob, position y (0-200) ,position x (0-320), transfert
*************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn AffDebecr(mob: c_short, y: c_short, x: c_short, transfere: c_short) {
    unsafe {
        let mut r = Rect::default();
        let mut gbak: CGrafPtr = std::ptr::null_mut();

        AcamGetPortOffscreen(&mut gbak);
        AcamSetPortOffscreen(debecr);
        Affmobbrt(mob, x, y, 0, std::ptr::null_mut());
        if transfere != 0 {
            CalculeAireMob(mob, x, y, &mut r);
            TfrEcr(&mut r);
        }
        AcamSetPortOffscreen(gbak);
    }
}
/*************************************************************************
                affomb
Affiche un mob sur l'ecran de travail (debecr) et le recopie sur le reel
=> Coordonnees en 320x200
*************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn affomb(
    mob: c_short,
    ligne: c_short,
    colonne: c_short,
    deltav: c_short,
    deltah: c_short,
    recopie: bool,
) {
    unsafe {
        let mut r = Rect::default();
        let mut gbak: CGrafPtr = std::ptr::null_mut();

        AcamGetPortOffscreen(&mut gbak);
        AcamSetPortOffscreen(debecr);
        Affmobbrtomb(mob, colonne, ligne, deltah, deltav, &mut r);
        if recopie {
            let rp = &r as *const Rect;
            CopyOffscreen(debecr, CarboniseGetWindowPort(mainWindow), rp, rp);
        }
        AcamSetPortOffscreen(gbak);
    }
}
/*************************************************************************
                AffmobPalette
Affiche un mob sur l'ecran virtuel, en entree on a en plus le tableau des
index de couleurs a utiliser
*************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn AffmobPalette(
    mob: c_short,
    y: c_short,
    x: c_short,
    inverse: c_short,
    tabIndex: *mut c_short,
) {
    unsafe {
        let mut gbak: CGrafPtr = std::ptr::null_mut();
        AcamGetPortOffscreen(&mut gbak);
        AcamSetPortOffscreen(Ecran1);
        Affmobbrt(mob, x, y, inverse, tabIndex);
        AcamSetPortOffscreen(gbak);
    }
}
/*************************************************************************
                AffPalette
Affiche la palette en cours
*************************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn AffPalette() {
    unsafe {
        let mut gbak: CGrafPtr = std::ptr::null_mut();
        let data: *mut u8;
        let mut tailleLigne: c_short = 0;
        let mut depth: c_short = 0;
        let mut rPort = Rect::default();

        AcamGetPortOffscreen(&mut gbak);
        AcamSetPortOffscreen(debecr);
        data = AcamGetInfoOffscreen(
            CarboniseGetCurrentPort(),
            &mut rPort,
            &mut tailleLigne,
            &mut depth,
        ) as *mut u8;
        if data.is_null() {
            return;
        }
        const TAILLE_CARRE: i64 = 16;
        for i in 0..16i16 {
            for j in 0..16i16 {
                let mut p = data.add(
                    (i as i64 * TAILLE_CARRE * tailleLigne as i64 + j as i64 * TAILLE_CARRE + 32)
                        as usize,
                );
                let coul = i as i32 * 16 + j as i32;
                for _k in 0..TAILLE_CARRE {
                    for l in 0..TAILLE_CARRE {
                        *p.add(l as usize) = coul as u8;
                    }
                    p = p.add(tailleLigne as usize);
                }
            }
        }
        TfrEcr(std::ptr::null_mut());
        AcamSetPortOffscreen(gbak);
    }
}
/***********************************************************************
               AffichePictDansAire
=> id de la picture,aire
***********************************************************************/
#[unsafe(no_mangle)]
pub extern "C" fn AffichePictDansAire(num: c_short, r: *mut Rect) {
    let mut rDest = Rect::default();
    let mut rpng = Rect::default();
    let mut pict: PicHandle = std::ptr::null_mut();
    let mut offscreen: CGrafPtr = std::ptr::null_mut();
    let mut nom = [0i8; 255];

    unsafe {
        DoZoomRectSiEcranReel(r, &mut rDest);
        PushPath();
        /* recherche de la picture en fichier */
        sprintf(
            nom.as_mut_ptr(),
            c"image_%d.png".as_ptr(),
            CvtDefaultLangRess(num) as c_int,
        );
        SetPathInData(c"images".as_ptr() as *mut c_char);
        if GetRectFilePng(nom.as_mut_ptr(), &mut rpng) {
            offscreen = ConvertPNGFileToOffscreen(nom.as_mut_ptr());
            if !offscreen.is_null() {
                BackColor(whiteColor as i16);
                ForeColor(blackColor as i16);
                CopyBits(
                    CarboniseGetPortBitMapForCopyBits(offscreen),
                    CarboniseGetPortBitMapForCopyBits(CarboniseGetCurrentPort()),
                    &rpng,
                    &rDest,
                    srcCopy as i16,
                    std::ptr::null_mut(),
                );
                AcamClosePortOffscreen(offscreen);
            }
        } else {
            pict = GetPicture(CvtDefaultLangRess(num));
            if pict.is_null() {
                pict = GetPicture(CvtLangRess(GB as c_short, num));
            }
            if !pict.is_null() {
                DrawPicture(pict, &rDest);
            }
        }
        PopPath();
    }
}

static mut TabPalette: [PalCoul; 16] = unsafe { std::mem::zeroed() };
static mut nbPalettes: i16 = 0;
#[unsafe(no_mangle)]
pub static mut TabMob: [Mob; 640usize] = unsafe { std::mem::zeroed() };
