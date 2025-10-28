use embassy_usb::{
    Handler,
    class::hid::{ReportId, RequestHandler},
    control::{InResponse, OutResponse, Request},
    types::{InterfaceNumber, StringIndex},
};

use crate::debug;

/// TODO: This handler does nothing expect log some info, that might be an issue.
pub struct OkeyRequestHandler;

impl RequestHandler for OkeyRequestHandler {
    fn get_report(&mut self, id: ReportId, _buf: &mut [u8]) -> Option<usize> {
        debug!(
            "Call to RequestHandler::get_report with id: {}. Doing nothing.",
            id,
        );
        None
    }

    fn set_report(&mut self, id: ReportId, _data: &[u8]) -> OutResponse {
        debug!(
            "Call to RequestHandler::set_report with id: {}. Doing nothing.",
            id,
        );
        OutResponse::Rejected
    }

    fn get_idle_ms(&mut self, id: Option<ReportId>) -> Option<u32> {
        debug!(
            "Call to RequestHandler::get_idle_ms with id: {}. Doing nothing.",
            id
        );
        None
    }

    fn set_idle_ms(&mut self, id: Option<ReportId>, duration_ms: u32) {
        debug!(
            "Call to RequestHandler::set_idle_ms with id: {}, duration_ms: {}. Doing nothing.",
            id, duration_ms
        )
    }
}

/// TODO: This handler does nothing except log some info, that might be an issue.
pub struct OkeyDeviceHandler;

impl Handler for OkeyDeviceHandler {
    fn enabled(&mut self, enabled: bool) {
        debug!(
            "Call to Handler::enabled with enabled: {}. Doing nothing.",
            enabled
        );
    }

    fn reset(&mut self) {
        debug!("Call to Handler::reset. Doing nothing.")
    }

    fn addressed(&mut self, addr: u8) {
        debug!(
            "Call to Handler::addressed with addr: {}. Doing nothing.",
            addr
        )
    }

    fn configured(&mut self, configured: bool) {
        debug!(
            "Call to Handler::configured with configured: {}. Doing nothing.",
            configured
        )
    }

    fn suspended(&mut self, suspended: bool) {
        debug!(
            "Call to Handler::suspended with suspended: {}. Doing nothing.",
            suspended
        )
    }

    fn remote_wakeup_enabled(&mut self, enabled: bool) {
        debug!(
            "Call to Handler::remove_wakeup_enabled with enabled: {}. Doing nothing.",
            enabled
        )
    }

    fn set_alternate_setting(&mut self, iface: InterfaceNumber, alternate_setting: u8) {
        debug!(
            "Call to Handler::set_alternate_setting with iface: {}, alternate_setting: {}. Doing nothing.",
            iface.0, alternate_setting
        );
    }

    fn control_out(&mut self, req: Request, _data: &[u8]) -> Option<OutResponse> {
        debug!(
            "Call to Handler::control_out with req of type: {}. Doing nothing.",
            req.request_type,
        );
        None
    }

    fn control_in<'a>(&'a mut self, req: Request, _buf: &'a mut [u8]) -> Option<InResponse<'a>> {
        debug!(
            "Call to Handler::control_in with req of type: {}. Doing nothing.",
            req.request_type,
        );
        None
    }

    fn get_string(&mut self, index: StringIndex, lang_id: u16) -> Option<&str> {
        debug!(
            "Call to Handler::get_string with index: {}, lang_id: {}. Doing nothing.",
            index.0, lang_id
        );
        None
    }
}
