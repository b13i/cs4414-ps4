/* kernel::sgash.rs */
#[allow(unused_imports)];
use core::*;
use core::str::*;
use core::option::{Some, Option, None}; 
use core::iter::Iterator;
use kernel::*;
use super::super::platform::*;
use kernel::memory::Allocator;
//use core::vec::Vec;

static mut BG_COLOR1: u32 = 0x00000000;
static mut FG_COLOR1: u32 = 0xffffffff;
static mut BG_COLOR2: u32 = 0xffffffff;
static mut FG_COLOR2: u32 = 0x00000000;
static mut BG_COLOR3: u32 = 0x6B6BFF00;
static mut FG_COLOR3: u32 = 0x00000000;
static mut CURRENT_COLOR_SCHEME: u32 = 1;

pub static mut prompt_buffer: rstr = rstr {
    p: 0 as *mut u8,
    p_str_i: 0,
    max_length: 0
};

pub fn putchar(key: char) {
    unsafe {
	/*
	 * We need to include a blank asm call to prevent rustc
	 * from optimizing this part out
	 */
	asm!("");
	io::write_char(key, io::UART0);
    }
}

fn putstr(msg: &str) {
    for c in slice::iter(as_bytes(msg)) {
	putchar(*c as char);
    }	
}

unsafe fn putrstr(s: rstr) {
    let mut p = s.p as uint;
    while *(p as *char) != '\0' {
        putchar( *(p as *char) );
        p += 1;
    }
}

unsafe fn drawrstr(s: rstr) {
    let mut p = s.p as uint;
    while *(p as *char) != '\0' {
        drawchar( *(p as *char) );
        p += 1;
    }
}

pub unsafe fn drawstr(msg: &str) {
    let old_fg = super::super::io::FG_COLOR;
    //let mut x: u32 = 0x6699AAFF;
	let mut x: u32 = 0x8238ADEC;
    for c in slice::iter(as_bytes(msg)) {
	x = (x << 8) + (x >> 24); 
	super::super::io::set_fg(x);
	drawchar(*c as char);
    }
    super::super::io::set_fg(old_fg);
}

unsafe fn drawchar(x: char)
{
    io::restore();
    if x == '\n' {
	io::CURSOR_Y += io::CURSOR_HEIGHT;
	io::CURSOR_X = 0u32;
    } else {
	io::draw_char(x);	
	io::CURSOR_X += io::CURSOR_WIDTH;
    }
    io::backup();
    io::draw_cursor();
}

unsafe fn backspace()
{
    io::restore();
    if (io::CURSOR_X >= io::CURSOR_WIDTH) { 
	io::CURSOR_X -= io::CURSOR_WIDTH;
	io::draw_char(' ');
    }
    io::backup();
    io::draw_cursor();

}

pub unsafe fn parsekey(x: char) {
    let x = x as u8;
    // Set this to false to learn the keycodes of various keys!
    // Key codes are printed backwards because life is hard

    match x { 
	13		=>	{
        parse();
        prompt();
       /*
        drawchar('\n');
        drawrstr(prompt_buffer);
	    parse();        
	    prompt_buffer.flush();
	    prompt();
        */
	    //putstr(&"\n");
	    //drawstr(&"\n");
	
	}
	127		=>	{ 
        if (prompt_buffer.remove_char()) {
	        putchar('');
	        putchar(' ');
	        putchar(''); 
	        backspace();
        }
	}
	_		=>	{ 
	    if io::CURSOR_X < io::SCREEN_WIDTH-io::CURSOR_WIDTH {
        prompt_buffer.add_char(x as u8);
		putchar(x as char);
		drawchar(x as char);
	    }
	}
    }
}

unsafe fn parse() {
	//putstr(&"\nAbout to parse:");
	//putrstr(prompt_buffer);
    //putstr(&"\nIn Parse");
    match prompt_buffer.get_arg_n(' ', 0) {
        Some(y) => {
            //putstr(&"\nFound Args: ");
            //putrstr(y);
			if y.streq(&"echo") {
				putstr(&"\necho echo echo echo echo...\n");
				drawstr(&"\necho echo echo echo echo...\n");
				match prompt_buffer.get_arg_n('"',1) {
					Some(x) => {
						drawrstr(x);
						putrstr(x);
						putstr(&"\n");
					},
					None => {}
				}
			}
			else if y.streq(&"ls") {
				/*
				match CWD {
					Some(cwd) => {cwd.list_dir();}
					None => {}
				}
				*/			
				//putstr(&"\n");
				
                match prompt_buffer.get_arg_n(' ', 1) {
                    Some(x) => {
                        //putstr(&"\nLS PARAMS: ");
                        //putrstr(x);
                        if (x.streq(&"-a")) {
                            putstr(&"\nList All");
                            drawstr(&"\nList All");
                        }
                        if (x.streq(&"-l")) {
                            putstr(&"\nList Long");
                            drawstr(&"\nList Long");
                        }
                    },
                    None => { putstr(&"\nListing"); drawstr(&"\nListing");}
                }
				
			}
			else if y.streq(&"cat") {
				putstr(&"\nmeow");
				drawstr(&"\nmeow");
			}
			else if y.streq(&"cd") {
				putstr(&"\nSomebody once told me the world is gunna roll me\nI ain't the sharpest tool in the shed.\nShe was looking kinda dumb with her finger and thumb\nin the shape of an L on her forehead...");
				drawstr(&"\nSomebody once told me the world is gunna roll me\nI ain't the sharpest tool in the shed.\nShe was looking kinda dumb with her finger and thumb\nin the shape of an L on her forehead...");
			}
			else if y.streq(&"rm") {
				putstr(&"\n Removing File");
				drawstr(&"\n Removing File");
			}
			else if y.streq(&"mkdir") {
				putstr(&"\n Creating Directory");
				drawstr(&"\n Creating Directory");
			}
			else if y.streq(&"pwd") {
				putstr(&"\n In Dir: Root");
				drawstr(&"\n In Dir: Root");
			}
			else if y.streq(&"wr") {
				putstr(&"\n Writing ");
				drawstr(&"\n Writing ");
				match prompt_buffer.get_arg_n('"', 1) {
					Some(x) => {
						putstr(&"\""); drawstr(&"\""); putrstr(x); drawrstr(x); putstr(&"\""); drawstr(&"\"");
					},
					None	=> { putstr(&"nothing"); drawstr(&"nothing"); }
				}
				putstr(&" to: ");
				drawstr(&" to: ");
				match prompt_buffer.get_arg_n(' ', 1) {
					Some(x) => { putrstr(x); drawrstr(x); }
					None 	=> { putstr(&"nil"); drawstr(&"nil"); }
				}
			} else if y.streq(&"mv") {
				putstr(&"\n Moving ");
				drawstr(&"\n Moving ");
				match prompt_buffer.get_arg_n(' ', 1) {
					Some(x) => { putrstr(x); drawrstr(x); },
					None	=> { putstr(&"nil"); drawstr(&"nil"); }
				}
				putstr(&" to ");
				drawstr(&" to ");
				match prompt_buffer.get_arg_n(' ', 2) {
					Some(x) => { putrstr(x); drawrstr(x); },
					None 	=> { putstr(&"nil"); drawstr(&"nil"); }
				}
			}
            else if y.streq(&"setcs") {
                //putstr(&"\nFOUND SETCS");
                
                match prompt_buffer.get_arg_n(' ', 1) {
                    Some(n) => {
                        //putstr(&"\nFound setcs num\n");
                        match (*(n.p as *char)) {
                            '1' => {set_color_scheme(1);},
                            '2' => {set_color_scheme(2);},
                            '3' => {set_color_scheme(3);},
                            _   => { }
                        }
/*
                        if (n.streq(&"1")) {
                            putstr(&"\nColor scheme 1");
                            set_color_scheme(1);
                        }
                        else if (n.streq(&"2")) {
                            putstr(&"\nColor scheme 2");
                            set_color_scheme(2);
                        }
                        else if (n.streq(&"3")) {
                            putstr(&"\nColor scheme 3");
                            set_color_scheme(3);
                        }
*/
                    },
                    None => {}
                }
     
            }
        },
        None => {
            putstr(&"\nNo Args");
        }
    };
    prompt_buffer.flush();
	//if cmd.streq(&"echo"){ 
    //    putstr(&"Trying to echo");
	//	drawrstr(args);
	//}
}
fn screen() {
    putstr(&"\n                                                               "); 
    putstr(&"\n                                                               ");
    putstr(&"\n                       7=..~$=..:7                             "); 
    putstr(&"\n                  +$: =$$$+$$$?$$$+ ,7?                        "); 
    putstr(&"\n                  $$$$$$$$$$$$$$$$$$Z$$                        ");
    putstr(&"\n              7$$$$$$$$$$$$. .Z$$$$$Z$$$$$$                    ");
    putstr(&"\n           ~..7$$Z$$$$$7+7$+.?Z7=7$$Z$$Z$$$..:                 ");
    putstr(&"\n          ~$$$$$$$$7:     :ZZZ,     :7ZZZZ$$$$=                ");
    putstr(&"\n           Z$$$$$?                    .+ZZZZ$$                 ");
    putstr(&"\n       +$ZZ$$$Z7                         7ZZZ$Z$$I.            "); 
    putstr(&"\n        $$$$ZZZZZZZZZZZZZZZZZZZZZZZZI,    ,ZZZ$$Z              "); 
    putstr(&"\n      :+$$$$ZZZZZZZZZZZZZZZZZZZZZZZZZZZ=    $ZZ$$+~,           "); 
    putstr(&"\n     ?$Z$$$$ZZZZZZZZZZZZZZZZZZZZZZZZZZZZI   7ZZZ$ZZI           "); 
    putstr(&"\n      =Z$$+7Z$$7ZZZZZZZZ$$$$$$$ZZZZZZZZZZ  ~Z$?$ZZ?            ");	 
    putstr(&"\n    :$Z$Z...$Z  $ZZZZZZZ~       ~ZZZZZZZZ,.ZZ...Z$Z$~          "); 
    putstr(&"\n    7ZZZZZI$ZZ  $ZZZZZZZ~       =ZZZZZZZ7..ZZ$?$ZZZZ$          "); 
    putstr(&"\n      ZZZZ$:    $ZZZZZZZZZZZZZZZZZZZZZZ=     ~$ZZZ$:           "); 
    putstr(&"\n    7Z$ZZ$,     $ZZZZZZZZZZZZZZZZZZZZ7         ZZZ$Z$          "); 
    putstr(&"\n   =ZZZZZZ,     $ZZZZZZZZZZZZZZZZZZZZZZ,       ZZZ$ZZ+         "); 
    putstr(&"\n     ,ZZZZ,     $ZZZZZZZ:     =ZZZZZZZZZ     ZZZZZ$:           "); 
    putstr(&"\n    =$ZZZZ+     ZZZZZZZZ~       ZZZZZZZZ~   =ZZZZZZZI          "); 
    putstr(&"\n    $ZZ$ZZZ$$Z$$ZZZZZZZZZ$$$$   IZZZZZZZZZ$ZZZZZZZZZ$          "); 
    putstr(&"\n      :ZZZZZZZZZZZZZZZZZZZZZZ   ~ZZZZZZZZZZZZZZZZZ~            "); 
    putstr(&"\n     ,Z$$ZZZZZZZZZZZZZZZZZZZZ    ZZZZZZZZZZZZZZZZZZ~           "); 
    putstr(&"\n     =$ZZZZZZZZZZZZZZZZZZZZZZ     $ZZZZZZZZZZZZZZZ$+           "); 
    putstr(&"\n        IZZZZZ:.                        . ,ZZZZZ$              "); 
    putstr(&"\n       ~$ZZZZZZZZZZZ                 ZZZZ$ZZZZZZZ+             "); 
    putstr(&"\n           Z$ZZZ. ,Z~               =Z:.,ZZZ$Z                 "); 
    putstr(&"\n          ,ZZZZZ..~Z$.             .7Z:..ZZZZZ:                ");
    putstr(&"\n          ~7+:$ZZZZZZZZI=:.   .,=IZZZZZZZ$Z:=7=                ");
    putstr(&"\n              $$ZZZZZZZZZZZZZZZZZZZZZZ$ZZZZ                    ");
    putstr(&"\n              ==..$ZZZ$ZZZZZZZZZZZ$ZZZZ .~+                    ");
    putstr(&"\n                  I$?.?ZZZ$ZZZ$ZZZI =$7                        ");
    putstr(&"\n                       $7..I$7..I$,                            ");
    putstr(&"\n"); 
    putstr(&"\n _                     _     _                         _  ");
    putstr(&"\n| |                   (_)   | |                       | | ");
    putstr(&"\n| | ____ ___  ____     _____| |_____  ____ ____  _____| | ");
    putstr(&"\n| |/ ___) _ \\|  _ \\   |  _   _) ___ |/ ___)  _ \\| ___ | | ");
    putstr(&"\n| | |  | |_| | | | |  | |  \\ \\| ____| |   | | | | ____| | ");
    putstr(&"\n|_|_|  \\____/|_| |_|  |_|   \\_\\_____)_|   |_| |_|_____)__)\n\n");
}

static mut ROOT : Option<file_node> = None;
static mut CWD : Option<file_node> = None;
pub unsafe fn init() {
    //prompt_buffer = rstr::new(256);
    prompt_buffer = rstr::new(256);
	screen();
    prompt();
    set_color_scheme(CURRENT_COLOR_SCHEME);
	//let mut test = rstr::new(256);
	//test.set_string(&"Testing..");
	//putrstr(test);
    let root_name : rstr = rstr::new(256);
    let root_dir : file_node = file_node {
        filename : root_name,
        next: None,
        child: None,
        num_children: 0 as uint,
        contents: root_name,
        node_type: DIR
    };
	ROOT = Some(root_dir);
	CWD = Some(root_dir);
}

unsafe fn prompt(){
	putstr(&"\nsgash> ");
	drawstr(&"\nsgash> ");
}

// File System Interface
/*
pub unsafe fn read_file(file) {

}

pub unsafe fn write_file(file, string) {

}
*/

unsafe fn set_color_scheme(scheme: u32) -> bool {
    CURRENT_COLOR_SCHEME = scheme;
    match CURRENT_COLOR_SCHEME {
        1 => {
            io::set_bg(BG_COLOR1);
            io::set_fg(FG_COLOR1);
            io::fill_bg();
        }
        2 => {
            io::set_bg(BG_COLOR2);
            io::set_fg(FG_COLOR2);
            io::fill_bg();
        }
        3 => {
            io::set_bg(BG_COLOR3);
            io::set_fg(FG_COLOR3);
            io::fill_bg();
        }
        _ => {}
    }
	true
}


/*enum file_node_type {
	FILE = 0,
	DIR = 1
}*/

static FILE: uint = 0;
static DIR: uint = 1;

struct file_node {
	
	filename: rstr,
	next: Option<*mut file_node>,
	child: Option<*mut file_node>,
	num_children: uint,
	contents: rstr,
	node_type: uint
}

impl file_node {
	
	unsafe fn get_filename (&mut self) -> rstr {
		self.filename
	}
	
	unsafe fn get_next (&self) -> Option<*mut file_node> {
		self.next
	}
	
	
	unsafe fn has_children(&self) -> bool {

		match self.child {
			Some(y) => {true},
			None    => {false}
			
		}
	}

	unsafe fn list_list (&self) {
		let n : file_node = *self;
		putrstr(n.filename);
		putstr("\n");
		drawrstr(n.filename);
		drawstr("\n");
		match n.next {
			Some(nn) => {let n2 = *nn; n2.list_list();},
			None => {}

		}
	}
	
	unsafe fn list_dir(&self) {
		match self.child {
			Some(n) => {
				let node : file_node = *n;
				node.list_list();
			},
			None =>{
				putstr(&"\nEmpty directory\n");
				drawstr(&"\nEmpty directory\n");	
			}

		}
	}

/*
	unsafe fn create_directory(&mut self, dir_name : rstr) -> bool {
		let empty : rstr = rstr::new(256);
		let new_dir : *mut file_node = file_node {
									filename: dir_name,
									next: None,
									child: None,
									num_children: 0,
									node_type: DIR,
									contents: empty};
		self.add_child(new_dir);
	}
*/	

	unsafe fn get_file(&self, fname: rstr) -> Option<file_node> {
		
		let mut temp = self.child;
		loop{
			match temp {
				Some(y) => {
					let x = *y;
					if x.filename.eq(fname) {return Some(x);}
					temp = (x.next);
				},
				None    => {
					return None;
				}
			}
		}
	}

	unsafe fn add_child(&mut self, new_file: *mut file_node) {
		if(self.node_type == DIR) {
			match self.child {
				Some(c) => {
					// Rearrange pointers
					let mut new_f = *new_file;
					new_f.next = Some(c);
					self.child = Some(new_file);
					self.num_children += 1;
				},
				None => {
					// Add pointer
					self.child = Some(new_file);	
					self.num_children += 1;					
				}
			}									
		}
	}

	unsafe fn remove_in_list(&mut self, fname: rstr) -> bool {
		let mut this : file_node = *self;		
		match this.next {
			Some(node) => {
				let mut n: file_node = *node;
				if (n.filename.eq(fname)) {
					this.next = n.next;
					true
				} else {
					return n.remove_in_list(fname);
				}
			},
			None => { false }
		}
		
	}

	unsafe fn remove_child(&mut self, fname: rstr) -> bool {
		let mut this: file_node = *self;		
		match self.child {
			Some(node) => { 
				let mut n : file_node = *node;
				if (n.filename.eq(fname)) {
					this.child = n.next;
					return true;
				} else {
					return n.remove_in_list(fname);
				}
			},
			None => { return false; }
		}
	}


}


//cwd
/*
unsafe fn read_file(file) {
	//root
	cwd.get_file(file);


}*/


struct rstr {
    p: *mut u8,
    max_length: uint,
    p_str_i: uint
}

impl rstr {
    pub unsafe fn new(size: uint) -> rstr {
        let (x,y) = heap.alloc(size);
        let this = rstr {
            p: x,
            max_length: y,
            p_str_i: 0
        };
        *(((this.p as uint)+this.p_str_i) as *mut char) = '\0';        
	    this
    }

#[allow(dead_code)]
	unsafe fn set_string(&mut self, value: &str) -> bool {
		self.flush();

		for c in slice::iter(as_bytes(value)) {
			self.add_char(*(c as *u8));
		}
		true
	}

#[allow(dead_code)]
    unsafe fn destroy(&self) { heap.free(self.p); }

    unsafe fn add_char(&mut self, x:u8) -> bool {
        if (self.p_str_i == self.max_length) { return false; }
        *(((self.p as uint)+self.p_str_i) as *mut u8) = x;
        self.p_str_i += 1;
        *(((self.p as uint)+self.p_str_i) as *mut char) = '\0';
        true
    }

    unsafe fn remove_char(&mut self) -> bool {
        if (self.p_str_i == 0) {
            // 0 length string return false
            return false;
        }
        self.p_str_i -= 1;
        *(((self.p as uint)+self.p_str_i) as *mut char) = '\0';
        true
    }

    unsafe fn eq(&self, other: rstr) -> bool {
		if (self.len() != other.len() ) {return false;}
		else {
			let mut x = 0;
			let mut selfindex = self.p as uint;
			let mut otherindex = other.p as uint;
			while (x < self.len()){
				if (*(selfindex as *char) != *(otherindex as *char)) 
					{return false;}
				selfindex += 1;
				otherindex += 1;
				x += 1;
			}
			return true;
		}
	}
    unsafe fn streq(&self, other: &str) -> bool {
	    let mut selfindex = self.p as uint;
	    for c in slice::iter(as_bytes(other)) {
		    if (*c != *(selfindex as *u8)) {return false;}
		    selfindex += 1;
	    }
	    *(selfindex as *char) == '\0'
    }

    unsafe fn get_arg_n(&self, delim: char, mut k: uint) -> Option<rstr> {
        let mut ind: uint = 0;
        let mut found = k == 0;
        let mut selfp: uint = self.p as uint;
        let mut s = rstr::new(256);
        loop {
            if (*(selfp as *char) == '\0') {
                if (found) { 
                    //putstr(&"\nFound str: ");
                    //putrstr(s);
                    return Some(s); }
                else { return None; }
            };
            if (*(selfp as *u8) == delim as u8) {
                if (found) { return Some(s); }
                k -= 1;
            };
            if (found) {
                s.add_char(*(selfp as *u8));
            };
            found = k == 0;
            selfp += 1;
            ind += 1;
            if (ind == self.max_length) {
                putstr(&"The Rent is too damn high\n");
                return None;
            }
        }
    }
/*
    unsafe fn get_arg_n(&self, delim: char, mut n: uint) -> Option<rstr> {
        putstr(&"\nGetting args for string: ");
        putrstr(*self);
        let mut index: uint = 0;
        let mut found = n == 0;
        putstr(&"\nMaking new string");
        let mut s = rstr::new(256);
        putstr(&"\nDone making string");
        let mut selfp : uint = self.p as uint;
        loop {
            putstr(&"\nLooping");
            if ( *(selfp as *char) == '\0' ) {
                if (found) { putstr("\nEnd: "); putrstr(s);  return Some(s); }
                return None;
            };
            if (*(selfp as *char) == delim) {
                if (found) {
					putstr(&"\nDelim: ");
					putrstr(s); 
                    return Some(s);
                }
                n -= 1;
            }
            if ( found ) {
                s.add_char( *(selfp as *u8) );
            }
            found = n == 0;
            selfp += 1;
            index += 1;
            if (index == self.max_length) {
                putstr(&"The rent is too damn high\n");
                return None;
            }
        }
    }
*/

/*
#[allow(dead_code)]
    unsafe fn split(&self, c: char) -> (rstr, rstr) {
    	let mut self_index: uint = self.p as uint;
        putstr(&"Creating\n");
	    let mut beg = rstr::new(256);
	    let mut end = rstr::new(256);
        putstr(&"Created\n");
	    let mut found = false;
	    loop {
            putstr(&"Looping\n");
		    if (*(self_index as *char) == '\0') {
                putstr(&"\nSplit Returning");
			    return (beg, end);		
		    }
		    else if(*(self_index as *u8) == c as u8) {
                putstr(&"\nFound delim");
			    found = true;		
		    }
		    else if(found) {
                putstr(&"\nAdding to end");
			    end.add_char(*(self_index as *u8));	
		    }    
		    else if(!found) {
                putstr(&"\nAdding to Begin");
		    	beg.add_char(*(self_index as *u8));
		    };
		    self_index += 1;
	    }
    }
*/

#[allow(dead_code)]
    unsafe fn split(&self, delim:char) -> (rstr, rstr) {
        let mut selfp: uint = self.p as uint;
        let mut beg = rstr::new(256);
        let mut end = rstr::new(256);
        let mut found = false;
        loop {
            if (*(selfp as *char) == '\0') {
                return (beg, end);
            }
            else if (*(selfp as *u8) == delim as u8) {
                found = true;
            }
            else if (!found) {
                beg.add_char(*(selfp as *u8));
            }
            else if (found) {
                end.add_char(*(selfp as *u8));
            };
            selfp += 1;
        }
    }

    unsafe fn len(&self) -> uint {
        self.p_str_i
    }

    unsafe fn flush(&mut self) {
        //putstr(&"\nFlushing Buffer");
        self.p_str_i = 0;
        *(self.p as *mut char) = '\0';
    }
}

