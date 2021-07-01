//! Based on SOLID/FMP3 `kernel.h`
use super::symbols;

pub struct Abi;

impl super::KernelAbi for Abi {
    fn get_symbols(&self, b: &mut super::SymbolsBuilder) {
        const TOPPERS_SUPPORT_DYNAMIC_CRE: bool = cfg!(feature = "dcre");
        const TOPPERS_SUPPORT_NGK_ALMCYC: bool = false; // not supported by us atm
        if TOPPERS_SUPPORT_DYNAMIC_CRE {
            b.insert_func(symbols::known_funcs::acre_tsk);
            b.insert_func(symbols::known_funcs::del_tsk);
        }
        b.insert_func(symbols::known_funcs::act_tsk);
        b.insert_func(symbols::known_funcs::mact_tsk);
        b.insert_func(symbols::known_funcs::can_act);
        b.insert_func(symbols::known_funcs::mig_tsk);
        b.insert_func(symbols::known_funcs::get_tst);
        b.insert_func(symbols::known_funcs::chg_pri);
        b.insert_func(symbols::known_funcs::get_pri);
        b.insert_func(symbols::known_funcs::get_inf);
        b.insert_func(symbols::known_funcs::chg_spr);
        b.insert_func(symbols::known_funcs::ref_tsk);
        b.insert_func(symbols::known_funcs::slp_tsk);
        b.insert_func(symbols::known_funcs::tslp_tsk);
        b.insert_func(symbols::known_funcs::wup_tsk);
        b.insert_func(symbols::known_funcs::can_wup);
        b.insert_func(symbols::known_funcs::rel_wai);
        b.insert_func(symbols::known_funcs::sus_tsk);
        b.insert_func(symbols::known_funcs::rsm_tsk);
        b.insert_func(symbols::known_funcs::dly_tsk);
        b.insert_func(symbols::known_funcs::ext_tsk);
        b.insert_func(symbols::known_funcs::ras_ter);
        b.insert_func(symbols::known_funcs::dis_ter);
        b.insert_func(symbols::known_funcs::ena_ter);
        b.insert_func(symbols::known_funcs::sns_ter);
        b.insert_func(symbols::known_funcs::ter_tsk);
        if TOPPERS_SUPPORT_DYNAMIC_CRE {
            b.insert_func(symbols::known_funcs::acre_sem);
            b.insert_func(symbols::known_funcs::del_sem);
        }
        b.insert_func(symbols::known_funcs::sig_sem);
        b.insert_func(symbols::known_funcs::wai_sem);
        b.insert_func(symbols::known_funcs::pol_sem);
        b.insert_func(symbols::known_funcs::twai_sem);
        b.insert_func(symbols::known_funcs::ini_sem);
        b.insert_func(symbols::known_funcs::ref_sem);
        if TOPPERS_SUPPORT_DYNAMIC_CRE {
            b.insert_func(symbols::known_funcs::acre_flg);
            b.insert_func(symbols::known_funcs::del_flg);
        }
        b.insert_func(symbols::known_funcs::set_flg);
        b.insert_func(symbols::known_funcs::clr_flg);
        b.insert_func(symbols::known_funcs::wai_flg);
        b.insert_func(symbols::known_funcs::pol_flg);
        b.insert_func(symbols::known_funcs::twai_flg);
        b.insert_func(symbols::known_funcs::ini_flg);
        b.insert_func(symbols::known_funcs::ref_flg);
        if TOPPERS_SUPPORT_DYNAMIC_CRE {
            b.insert_func(symbols::known_funcs::acre_dtq);
            b.insert_func(symbols::known_funcs::del_dtq);
        }
        b.insert_func(symbols::known_funcs::snd_dtq);
        b.insert_func(symbols::known_funcs::psnd_dtq);
        b.insert_func(symbols::known_funcs::tsnd_dtq);
        b.insert_func(symbols::known_funcs::fsnd_dtq);
        b.insert_func(symbols::known_funcs::rcv_dtq);
        b.insert_func(symbols::known_funcs::prcv_dtq);
        b.insert_func(symbols::known_funcs::trcv_dtq);
        b.insert_func(symbols::known_funcs::ini_dtq);
        b.insert_func(symbols::known_funcs::ref_dtq);
        if TOPPERS_SUPPORT_DYNAMIC_CRE {
            b.insert_func(symbols::known_funcs::acre_pdq);
            b.insert_func(symbols::known_funcs::del_pdq);
        }
        b.insert_func(symbols::known_funcs::snd_pdq);
        b.insert_func(symbols::known_funcs::psnd_pdq);
        b.insert_func(symbols::known_funcs::tsnd_pdq);
        b.insert_func(symbols::known_funcs::rcv_pdq);
        b.insert_func(symbols::known_funcs::prcv_pdq);
        b.insert_func(symbols::known_funcs::trcv_pdq);
        b.insert_func(symbols::known_funcs::ini_pdq);
        b.insert_func(symbols::known_funcs::ref_pdq);
        if TOPPERS_SUPPORT_DYNAMIC_CRE {
            b.insert_func(symbols::known_funcs::acre_mtx);
            b.insert_func(symbols::known_funcs::del_mtx);
        }
        b.insert_func(symbols::known_funcs::loc_mtx);
        b.insert_func(symbols::known_funcs::ploc_mtx);
        b.insert_func(symbols::known_funcs::tloc_mtx);
        b.insert_func(symbols::known_funcs::unl_mtx);
        b.insert_func(symbols::known_funcs::ini_mtx);
        b.insert_func(symbols::known_funcs::ref_mtx);
        if TOPPERS_SUPPORT_DYNAMIC_CRE {
            b.insert_func(symbols::known_funcs::acre_spn);
            b.insert_func(symbols::known_funcs::del_spn);
        }
        b.insert_func(symbols::known_funcs::loc_spn);
        b.insert_func(symbols::known_funcs::unl_spn);
        b.insert_func(symbols::known_funcs::try_spn);
        b.insert_func(symbols::known_funcs::ref_spn);
        if TOPPERS_SUPPORT_DYNAMIC_CRE {
            b.insert_func(symbols::known_funcs::acre_mpf);
            b.insert_func(symbols::known_funcs::del_mpf);
        }
        b.insert_func(symbols::known_funcs::get_mpf);
        b.insert_func(symbols::known_funcs::pget_mpf);
        b.insert_func(symbols::known_funcs::tget_mpf);
        b.insert_func(symbols::known_funcs::rel_mpf);
        b.insert_func(symbols::known_funcs::ini_mpf);
        b.insert_func(symbols::known_funcs::ref_mpf);
        b.insert_func(symbols::known_funcs::set_tim);
        b.insert_func(symbols::known_funcs::get_tim);
        b.insert_func(symbols::known_funcs::adj_tim);
        b.insert_func(symbols::known_funcs::fch_hrt);
        if TOPPERS_SUPPORT_DYNAMIC_CRE {
            b.insert_func(symbols::known_funcs::acre_cyc);
            b.insert_func(symbols::known_funcs::del_cyc);
            if TOPPERS_SUPPORT_NGK_ALMCYC {
                b.insert_func(symbols::known_funcs::acre_cyc_ngk);
            }
        }
        b.insert_func(symbols::known_funcs::sta_cyc);
        b.insert_func(symbols::known_funcs::msta_cyc);
        b.insert_func(symbols::known_funcs::stp_cyc);
        b.insert_func(symbols::known_funcs::ref_cyc);
        if TOPPERS_SUPPORT_DYNAMIC_CRE {
            b.insert_func(symbols::known_funcs::acre_alm);
            b.insert_func(symbols::known_funcs::del_alm);
            if TOPPERS_SUPPORT_NGK_ALMCYC {
                b.insert_func(symbols::known_funcs::acre_alm_ngk);
            }
        }
        b.insert_func(symbols::known_funcs::sta_alm);
        b.insert_func(symbols::known_funcs::msta_alm);
        b.insert_func(symbols::known_funcs::stp_alm);
        b.insert_func(symbols::known_funcs::ref_alm);
        b.insert_func(symbols::known_funcs::rot_rdq);
        b.insert_func(symbols::known_funcs::mrot_rdq);
        b.insert_func(symbols::known_funcs::get_tid);
        b.insert_func(symbols::known_funcs::get_pid);
        b.insert_func(symbols::known_funcs::get_lod);
        b.insert_func(symbols::known_funcs::mget_lod);
        b.insert_func(symbols::known_funcs::get_nth);
        b.insert_func(symbols::known_funcs::mget_nth);
        b.insert_func(symbols::known_funcs::loc_cpu);
        b.insert_func(symbols::known_funcs::unl_cpu);
        b.insert_func(symbols::known_funcs::dis_dsp);
        b.insert_func(symbols::known_funcs::ena_dsp);
        b.insert_func(symbols::known_funcs::sns_ctx);
        b.insert_func(symbols::known_funcs::sns_loc);
        b.insert_func(symbols::known_funcs::sns_dsp);
        b.insert_func(symbols::known_funcs::sns_dpn);
        b.insert_func(symbols::known_funcs::sns_ker);
        b.insert_func(symbols::known_funcs::ext_ker);
        if TOPPERS_SUPPORT_DYNAMIC_CRE {
            b.insert_func(symbols::known_funcs::acre_isr);
            b.insert_func(symbols::known_funcs::del_isr);
        }
        b.insert_func(symbols::known_funcs::dis_int);
        b.insert_func(symbols::known_funcs::ena_int);
        b.insert_func(symbols::known_funcs::clr_int);
        b.insert_func(symbols::known_funcs::ras_int);
        b.insert_func(symbols::known_funcs::prb_int);
        b.insert_func(symbols::known_funcs::chg_ipm);
        b.insert_func(symbols::known_funcs::get_ipm);
        b.insert_func(symbols::known_funcs::xsns_dpn);

        // Not in `kernel.h`
        if cfg!(feature = "exd_tsk") {
            b.insert_func(symbols::known_funcs::exd_tsk);
        }
    }
}
