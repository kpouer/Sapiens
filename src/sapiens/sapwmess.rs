/****************************************************************************
            Encadre et efface le message
*****************************************************************************/
use crate::sapiens::sapwbrt::{CopyOffscreen, DoEraseRect, DoFrameRect, DoPenSize, DoScrollRect};
use crate::shared::Rect;
use crate::stubs::CarboniseInvalRect;
use crate::{Sapiens, port};

pub(crate) struct Sapwmess {
    pub(crate) rMess: Rect,
}

impl Default for Sapwmess {
    fn default() -> Self {
        Self {
            rMess: Rect {
                top: 208,
                left: 32,
                bottom: 224,
                right: 606,
            },
        }
    }
}

impl Sapiens {
    pub(crate) fn framebadass(&self) {
        let _g = port::on(self.sapwind.debecr);
        let mut r = self.sapwmess.rMess;
        r.InsetRect(-1, -1);
        DoEraseRect(&r);
        DoFrameRect(&r);
        DoPenSize(1, 1);
    }

    /****************************************************************************
                TfrBadAss
    Le message s'affiche sur debecr, on le transfere sur le reel
    *****************************************************************************/
    #[unsafe(no_mangle)]
    pub(crate) extern "C" fn TfrBadAss(&mut self) {
        self.eff_sour();
        CopyOffscreen(
            self.sapwind.debecr,
            self.sapwind.mainWindow,
            &raw mut self.sapwmess.rMess,
            &raw mut self.sapwmess.rMess,
        );
        CarboniseInvalRect(&raw const self.sapwmess.rMess);
    }

    /****************************************************************************
                Effacement du message
    *****************************************************************************/
    pub(crate) fn efbadass(&self) {
        let _g = port::on(self.sapwind.debecr);
        DoEraseRect(&self.sapwmess.rMess);
    }

    /****************************************************************************
                Scrolling du message
    *****************************************************************************/
    pub(crate) fn scrollbadass(&mut self) {
        let mut i;
        self.eff_sour();
        let _g = port::on(self.sapwind.mainWindow);
        let reff = Rect::new(
            self.sapwmess.rMess.left,
            self.sapwmess.rMess.top,
            self.sapwmess.rMess.left + 8,
            self.sapwmess.rMess.bottom,
        );
        i = 0;
        while i < self.sapwmess.rMess.right - self.sapwmess.rMess.left {
            DoScrollRect(&self.sapwmess.rMess, 8, 0, core::ptr::null_mut());
            DoEraseRect(&reff);
            i += 8;
        }
        self.efbadass();
    }
}
