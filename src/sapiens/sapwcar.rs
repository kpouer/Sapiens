#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
use crate::langue::Langue::ES;
use crate::langue::{Langue, langue};
use crate::port::PortId;
use crate::sapiens::Sapiens;
use crate::sapiens::sapcolorcvt::SapColor2Index;
use crate::sapiens::saptxt::{CvtCar, CvtCarLissage};
use crate::sapiens::{POLICE_LISSAGE, STYLE_LISSAGE};
use crate::shared::{Boolean, Point, RGBColor, Rect};
use crate::stubs::CarboniseSetPatBlack;

const ECHELLE: i16 = 2;

/* Etat mutable de SAPWCAR.C, ex-`static mut`, regroupe dans cette
structure, champ de `Sapiens`. */
pub(crate) struct Sapwcar {
    /* ex-`LissagePolice` */
    pub(crate) lissage_police: Boolean,
    /* ex-`indexWhite` */
    index_white: i16,
    /* ex-`indexBlack` */
    index_black: i16,
    /* ex-`indexBrown` */
    index_brown: i16,
    /* ex-`indexGray` */
    index_gray: i16,
    /* ex-`ptEcrCar` */
    pt_ecr_car: Point,
}

impl Default for Sapwcar {
    fn default() -> Self {
        Self {
            lissage_police: true,
            index_white: 0,
            index_black: 0,
            index_brown: 0,
            index_gray: 0,
            pt_ecr_car: Point { v: 0, h: 0 },
        }
    }
}

static rgbWhite: RGBColor = RGBColor {
    red: 0xFFFF,
    green: 0xFFFF,
    blue: 0xFFFF,
};
static rgbBlack: RGBColor = RGBColor {
    red: 0,
    green: 0,
    blue: 0,
};
static rgbBrown: RGBColor = RGBColor {
    red: 34368,
    green: 27840,
    blue: 7008,
};
static rgbGray: RGBColor = RGBColor {
    red: 50000,
    green: 50000,
    blue: 50000,
};

impl Sapiens {
    /*************************************************************************
                        InitCouleurFontes
    **************************************************************************/
    pub(crate) fn InitCouleurFontes(&mut self) {
        let _g = self.sapwind.ports.on(PortId::Ecran1);
        self.sapwcar.index_white = SapColor2Index(&rgbWhite);
        self.sapwcar.index_black = SapColor2Index(&rgbBlack);
        self.sapwcar.index_brown = SapColor2Index(&rgbBrown);
        self.sapwcar.index_gray = SapColor2Index(&rgbGray);
    }

    /*************************************************************************
                        Gotoxy
    Positionne l'ecriture des caracteres en y et x
    => position en colonne et ligne caracteres
    **************************************************************************/
    pub(crate) fn Gotoxy(&mut self, x: i16, y: i16) {
        self.sapwcar.pt_ecr_car = Point::new((x * ECHELLE) * 6, (y * ECHELLE) * 7 - 1);
        self.sapglob.colaff = x;
        self.sapglob.linaff = y;
    }
}

impl Sapwcar {
    /*************************************************************************
                        GotoxyPixel
    Positionne l'ecriture des caracteres en y et x pixel
    => position en colonne et ligne caracteres
    **************************************************************************/
    pub(crate) fn GotoxyPixel(&mut self, x: i16, y: i16) {
        self.pt_ecr_car = Point::new(x, y);
    }
}

impl Sapiens {
    /*************************************************************************
                        TailleCar
    **************************************************************************/
    pub(crate) fn TailleCar(&mut self, car: i16, taille: i16) -> i16 {
        if self.sapwcar.lissage_police || langue() == ES {
            /* ES Pour les caracteres speciaux */
            let car = CvtCarLissage(car);
            self.sapwind.ports.TextFont(POLICE_LISSAGE as i16);
            self.sapwind.ports.TextSize(taille);
            self.sapwind.ports.TextFace(STYLE_LISSAGE as i16);
            self.sapwind.ports.CharWidth(car as i8)
        } else {
            let car = CvtCar(car);
            if taille == 10 {
                let num = self.sapwind.Fonte10 + car - 31;
                self.sapwbrt.TabMob[num as usize].nbcol + 2
            } else {
                let num = self.sapwind.Fonte14 + car - 31;
                self.sapwbrt.TabMob[num as usize].nbcol + 3
            }
        }
    }
}

/*************************************************************************
                    SetColorFont
**************************************************************************/
fn SetColorFont(sapiens: &mut Sapiens, id: i16) {
    if id == sapiens.sapwcar.index_white {
        sapiens.sapwind.ports.RGBForeColor(&rgbWhite);
    } else if id == sapiens.sapwcar.index_black {
        sapiens.sapwind.ports.RGBForeColor(&rgbBlack);
    } else if id == sapiens.sapwcar.index_brown {
        sapiens.sapwind.ports.RGBForeColor(&rgbBrown);
    } else if id == sapiens.sapwcar.index_gray {
        sapiens.sapwind.ports.RGBForeColor(&rgbGray);
    } else {
        sapiens.Collapse(format!("SetColorFont: {id} unknown"));
    }
}

impl Sapiens {
    /*************************************************************************
                        AffCarBrt
    Affiche un caractere brut à la position courante
    => Caractere à afficher
    **************************************************************************/
    fn AffCarBrt(&mut self, car: i16, couleur: i16, couleurOmbre: i16, taille: i16) {
        if self.sapwcar.lissage_police || langue() == Langue::ES {
            /* ES Pour les caracteres speciaux */
            let mut dx: i16 = 0;
            let dy: i16 = 0;
            let mut car = CvtCarLissage(car);
            if car == '<' as i16 {
                car = 0xC9;
                dx -= 1;
            }
            if car == '>' as i16 {
                car = 0xC9;
                dx -= 1;
            }
            self.sapwind.ports.TextFont(POLICE_LISSAGE as i16);
            self.sapwind.ports.TextSize(taille);
            self.sapwind.ports.TextFace(STYLE_LISSAGE as i16);
            let info = self.sapwind.ports.GetFontInfo();
            SetColorFont(self, couleurOmbre);
            self.sapwind.ports.MoveTo(
                self.sapwcar.pt_ecr_car.h + dx,
                self.sapwcar.pt_ecr_car.v + 1 + info.ascent + dy + 1,
            );
            self.sapwind.ports.DrawChar(car as i8);
            SetColorFont(self, couleur);
            self.sapwind.ports.MoveTo(
                self.sapwcar.pt_ecr_car.h + dx,
                self.sapwcar.pt_ecr_car.v + info.ascent + dy,
            );
            self.sapwind.ports.DrawChar(car as i8);
            self.sapwcar.pt_ecr_car.h += self.sapwind.ports.CharWidth(car as i8);
        } else {
            let car = CvtCar(car);
            let num: i16;
            if taille == 10 {
                num = self.sapwind.Fonte10 + car - 31;
                self.AffmobCar(
                    num,
                    self.sapwcar.pt_ecr_car.h + 1,
                    self.sapwcar.pt_ecr_car.v + 1,
                    couleurOmbre,
                );
                self.AffmobCar(
                    num,
                    self.sapwcar.pt_ecr_car.h,
                    self.sapwcar.pt_ecr_car.v,
                    couleur,
                );
                self.sapwcar.pt_ecr_car.h += self.sapwbrt.TabMob[num as usize].nbcol + 2;
            } else {
                num = self.sapwind.Fonte14 + car - 31;
                self.AffmobCar(
                    num,
                    self.sapwcar.pt_ecr_car.h + 2,
                    self.sapwcar.pt_ecr_car.v + 1,
                    couleurOmbre,
                );
                self.AffmobCar(
                    num,
                    self.sapwcar.pt_ecr_car.h + 1,
                    self.sapwcar.pt_ecr_car.v,
                    couleur,
                );
                self.AffmobCar(
                    num,
                    self.sapwcar.pt_ecr_car.h,
                    self.sapwcar.pt_ecr_car.v,
                    couleur,
                );
                self.sapwcar.pt_ecr_car.h += self.sapwbrt.TabMob[num as usize].nbcol + 3;
            }
        }
    }
}

/*************************************************************************
                    GetStrBkMot
**************************************************************************/
impl Sapiens {
    pub(crate) fn GetStrBkMot(&self, car: i16) -> Vec<u8> {
        let car = car - 129;

        let Some(mot) = usize::try_from(car)
            .ok()
            .and_then(|index| self.sap_lang.GetBkMot().get(index))
        else {
            eprintln!("GetStrBkMot error {car}");
            return vec![0];
        };

        if mot.first() == Some(&b'$') {
            /* Substitution */
            match mot.get(1) {
                Some(b'0') => self.saptxt.nomsecondaire.to_vec(),
                Some(b'1') => self.saptxt.nomjoueur.to_vec(),
                Some(b'2') => self.saptxt.nomadversaire.to_vec(),
                Some(b'3') => self.saptxt.nomtribu.to_vec(),
                Some(b'4') => self.saptxt.nomcopain.to_vec(),
                _ => mot.to_vec(),
            }
        } else {
            mot.to_vec()
        }
    }

    /*************************************************************************
                        AffCarBank
    Affiche un caractere sur le port en entrée
    **************************************************************************/
    fn AffCarBank(&mut self, port: PortId, car: i16, couleur1: i16, couleur2: i16, taille: i16) {
        let _g = self.sapwind.ports.on(port);
        if car > 128 {
            let mut p = 0;
            let s = self.GetStrBkMot(car);
            while s[p] != 0 {
                self.AffCarBank(port, s[p] as i16, couleur1, couleur2, taille);
                p += 1;
            }
        } else {
            self.AffCarBrt(car, couleur1, couleur2, taille);
            self.sapglob.colaff += 1;
        }
    }

    /*************************************************************************
                        affcar
    Affiche le caractere demandé à la position courante
    Les caracteres >128 sont des macros substitions
    **************************************************************************/
    pub(crate) fn affcar(&mut self, car: i16) {
        self.AffCarBank(
            PortId::Debecr,
            car,
            self.sapwcar.index_black,
            self.sapwcar.index_brown,
            14,
        );
    }

    /*************************************************************************
                        affcar10
    Affiche le caractere demandé à la position courante
    Les caracteres >128 sont des macros substitions
    **************************************************************************/
    pub(crate) fn affcar10(&mut self, car: i16) {
        self.AffCarBank(
            PortId::Debecr,
            car,
            self.sapwcar.index_black,
            self.sapwcar.index_gray,
            10,
        );
    }

    /*************************************************************************
                        Car2
    Affiche un caractere blanc sur une ombre noire
    Le caractere est affiché sur Debecr et n'est pas transféré
    **************************************************************************/
    pub(crate) fn car2(&mut self, car: i16) {
        self.AffCarBank(
            PortId::Debecr,
            car,
            self.sapwcar.index_white,
            self.sapwcar.index_black,
            14,
        );
    }

    /*************************************************************************
                        affcarmess
    Affiche le caractere demandé à la position courante
    Les caracteres >128 sont des macros substitions
    Le caractere est affiche sur Debecr
    **************************************************************************/
    pub(crate) fn affcarmess(&mut self, car: i16) {
        let _g = self.sapwind.ports.on(PortId::Debecr);
        let rgn = self.sapwind.ports.GetClip();
        self.sapwind.ports.ClipRect(&self.sapwmess.rMess);
        /* Effacement et affichage curseur */
        if car < 32 {
            let r = Rect::new(
                self.sapwcar.pt_ecr_car.h,
                self.sapwcar.pt_ecr_car.v,
                self.sapwcar.pt_ecr_car.h + 8,
                self.sapwcar.pt_ecr_car.v + 11,
            );
            if car == 2 {
                CarboniseSetPatBlack();
                self.sapwind.ports.DoPaintRect(&r);
            }
            if car == 1 {
                self.sapwind.ports.DoEraseRect(&r);
            }
        } else {
            self.AffCarBank(
                PortId::Debecr,
                car,
                self.sapwcar.index_black,
                self.sapwcar.index_gray,
                14,
            );
        }
        self.sapwind.ports.SetClip(&rgn);
    }

    /***********************************************************************
                            MOTLST2:Affiche Pour jauges
    ***********************************************************************/
    pub(crate) fn motlst2(&mut self, debut_index: usize) {
        let mut p = 0;
        while self.sap_lang.GetApp()[debut_index][p] != 0 {
            self.car2(self.sap_lang.GetApp()[debut_index][p] as i16);
            p += 1;
        }
    }
}
