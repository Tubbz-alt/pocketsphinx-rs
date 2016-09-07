extern crate pocketsphinx;

use std::fs::File;
use std::io::Read;

fn test() -> Result<(),pocketsphinx::Error> {

    let ps_config = try!(pocketsphinx::CmdLn::init(true, &["pocketsphinx",
          "-hmm", "data/en-us",
          "-lm", "data/en-us.lm.bin",
          "-dict", "data/cmudict-en-us.dict",
        ]));
    let ps_decoder = pocketsphinx::PsDecoder::init(ps_config);

    let mut f = File::open(&"data/goforward.raw").unwrap();
    let mut buffer = [0; 2048];
    let mut samples : [i16; 1024];

    try!(ps_decoder.start_utt(Some("something")));
    loop {    
	let n = f.read(&mut buffer).unwrap();
	if n == 0 {
	    break;
	}
	samples = unsafe {std::mem::transmute(buffer)};
        try!(ps_decoder.process_raw(&mut samples, false, false));
    }
    try!(ps_decoder.end_utt());
    match ps_decoder.get_hyp() {
            None => println!("Not recognized"),
            Some((hyp, _utt_id, _score)) => println!("Recognized: {}", hyp),
    }
    Ok(())
}

fn main() {
    match test() {
        Result::Ok(_) => println!("Done!"),
        Result::Err(_) => println!("Error!")
    }
}
