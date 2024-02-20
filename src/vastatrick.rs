use std::{sync::{Arc, mpsc::{Sender, Receiver, self}}, collections::HashMap};

use crate::{class::Class, VastatrixRequest, VastatrixResponse, VastatrixRequestKind};

//one singular unit of the collection of vastatrix.
pub struct Vastatrick {
    cache: HashMap<String, Arc<dyn Class>>, //is this necessary?
    requester: Sender<VastatrixRequest>,
    response_sender: Arc<Sender<VastatrixResponse>>,
    responses: Receiver<VastatrixResponse>,
}

impl Vastatrick {
    pub fn new(requester: Sender<VastatrixRequest>) -> Self {
	let (response_sender, responses) = mpsc::channel();
	Self {
	    cache: HashMap::new(),
	    requester,
	    response_sender: Arc::new(response_sender),
	    responses,
	}
    }

    pub fn run_main_class(&mut self, classpath: String) {
	let main_class = self.send_request(VastatrixRequestKind::ResolveClass(classpath));
	
	self.send_request(VastatrixRequestKind::Exit(0));
    }

    fn send_request(&self, request: VastatrixRequestKind) -> VastatrixResponse {
	self.requester.send(VastatrixRequest {
	    responder: Arc::clone(&self.response_sender),
	    kind: request,
	}).unwrap();
	return self.responses.recv().unwrap()
    }
}
