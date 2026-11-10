/****************************************************************************
            Encadre et efface le message
*****************************************************************************/
use crate::port::PortId;
use crate::sapiens::Sapiens;
use crate::shared::Rect;
use crate::stubs::CarboniseInvalRect;

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
    pub(crate) fn framebadass(&mut self) {
        let _g = self.sapwind.ports.on(PortId::Debecr);
        let mut r = self.sapwmess.rMess;
        r.InsetRect(-1, -1);
        self.sapwind.ports.DoEraseRect(&r);
        self.sapwind.ports.DoFrameRect(&r);
        self.sapwind.ports.DoPenSize(1, 1);
    }

    /****************************************************************************
                TfrBadAss
    Le message s'affiche sur Debecr, on le transfere sur le reel
    *****************************************************************************/
    pub(crate) fn TfrBadAss(&mut self) {
        self.eff_sour();
        self.sapwind.ports.CopyOffscreen(
            PortId::Debecr,
            PortId::MainWindow,
            self.sapwmess.rMess,
            &self.sapwmess.rMess,
        );
        CarboniseInvalRect(&self.sapwmess.rMess);
    }

    /****************************************************************************
                Effacement du message
    *****************************************************************************/
    pub(crate) fn efbadass(&mut self) {
        let _g = self.sapwind.ports.on(PortId::Debecr);
        self.sapwind.ports.DoEraseRect(&self.sapwmess.rMess);
    }

    /****************************************************************************
                Scrolling du message
    *****************************************************************************/
    pub(crate) fn scrollbadass(&mut self) {
        let mut i;
        self.eff_sour();
        let _g = self.sapwind.ports.on(PortId::MainWindow);
        let reff = Rect::new(
            self.sapwmess.rMess.left,
            self.sapwmess.rMess.top,
            self.sapwmess.rMess.left + 8,
            self.sapwmess.rMess.bottom,
        );
        i = 0;
        while i < self.sapwmess.rMess.right - self.sapwmess.rMess.left {
            self.sapwind.ports.DoScrollRect(&self.sapwmess.rMess, 8, 0);
            self.sapwind.ports.DoEraseRect(&reff);
            i += 8;
        }
        self.efbadass();
    }
}
