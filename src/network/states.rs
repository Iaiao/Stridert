#[derive(PartialEq)]
pub enum State {
	HANDSHAKING,
	STATUS,
	LOGIN,
	PLAY
}