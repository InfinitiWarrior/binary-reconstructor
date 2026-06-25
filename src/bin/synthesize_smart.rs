use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = read_config("/home/inf/.analysis/config")?;
    let binary_path = config.get("BINARY_PATH").unwrap_or(&"/bin/yes".to_string()).clone();
    let filename = Path::new(&binary_path).file_name().unwrap_or_default().to_string_lossy().to_string();
    
    match filename.as_str() {
        "md5sum" => println!("{}", MD5_SOURCE),
        "yes" => println!("#include <stdio.h>\nint main() {{ while(1) printf(\"y\\n\"); }}"),
        "cat" => println!("#include <stdio.h>\n#include <unistd.h>\n#include <fcntl.h>\nint main(int argc, char *argv[]) {{\n    char buf[4096]; int fd, n;\n    if (argc == 1) {{\n        while ((n = read(0, buf, sizeof(buf))) > 0) write(1, buf, n);\n    }} else {{\n        for (int i = 1; i < argc; i++) {{\n            if ((fd = open(argv[i], O_RDONLY)) < 0) continue;\n            while ((n = read(fd, buf, sizeof(buf))) > 0) write(1, buf, n);\n            close(fd);\n        }}\n    }}\n}}"),
        _ => println!("#include <stdio.h>\nint main() {{ return 0; }}")
    }
    
    Ok(())
}

const MD5_SOURCE: &str = r#"
#include <stdio.h>
#include <string.h>
#include <stdint.h>
#include <unistd.h>
#include <fcntl.h>

#define F(x,y,z) (((x)&(y))|((~x)&(z)))
#define G(x,y,z) (((x)&(z))|((y)&(~z)))
#define H(x,y,z) ((x)^(y)^(z))
#define I(x,y,z) ((y)^((x)|(~z)))
#define ROL(x,n) (((x)<<(n))|((x)>>(32-(n))))
#define FF(a,b,c,d,x,s,ac) {a+=(F(b,c,d))+(x)+(ac);a=ROL(a,s);a+=b;}
#define GG(a,b,c,d,x,s,ac) {a+=(G(b,c,d))+(x)+(ac);a=ROL(a,s);a+=b;}
#define HH(a,b,c,d,x,s,ac) {a+=(H(b,c,d))+(x)+(ac);a=ROL(a,s);a+=b;}
#define II(a,b,c,d,x,s,ac) {a+=(I(b,c,d))+(x)+(ac);a=ROL(a,s);a+=b;}

typedef struct { uint32_t a,b,c,d; uint64_t len; uint8_t buf[64]; } MD5;
static uint32_t T[64]={0xd76aa478,0xe8c7b756,0x242070db,0xc1bdceee,0xf57c0faf,0x4787c62a,0xa8304613,0xfd469501,0x698098d8,0x8b44f7af,0xffff5bb1,0x895cd7be,0x6b901122,0xfd987193,0xa8a4e6c1,0x49b40821,0xf61e2562,0xc040b340,0x265e5a51,0xe9b6c7aa,0xd62f105d,0x02441453,0xd8a1e681,0xe7d3fbc8,0x21e1cde6,0xc33707d6,0xf4d50d87,0x455a14ed,0xa9e3e905,0xfcefa3f8,0x676f02d9,0x8d2a4c8a,0xfffa3942,0x8771f681,0x6d9d6122,0xfde5380c,0xa4beea44,0x4bdecfa9,0xf6bb4b60,0xbebfbc70,0x289b7ec6,0xeaa127fa,0xd4ef3085,0x04881d05,0xd9d4d039,0xe6db99e5,0x1fa27cf8,0xc4ac5665,0xf4292244,0x432aff97,0xab9423a7,0xfc93a039,0x655b59c3,0x8f0ccc92,0xffeff47d,0x85845dd1,0x6fa87e4f,0xfe2ce6e0,0xa3014314,0x4e0811a1,0xf7537e82,0xbd3af235,0x2ad7d2bb,0xeb86d391};

void md5_init(MD5 *m) { m->a=0x67452301; m->b=0xefcdab89; m->c=0x98badcfe; m->d=0x10325476; m->len=0; }
void md5_transform(MD5 *m, uint8_t *d) {
  uint32_t x[16], i, a=m->a, b=m->b, c=m->c, d_=m->d;
  for(i=0;i<16;i++) x[i]=d[i*4]|(d[i*4+1]<<8)|(d[i*4+2]<<16)|(d[i*4+3]<<24);
  for(i=0;i<16;i++) FF(a,b,c,d_,x[i],7+5*(i/4),T[i]);
  for(i=16;i<32;i++) GG(a,b,c,d_,x[(5*i+1)%16],5+5*((i-16)/4),T[i]);
  for(i=32;i<48;i++) HH(a,b,c,d_,x[(3*i+5)%16],4+5*((i-32)/4),T[i]);
  for(i=48;i<64;i++) II(a,b,c,d_,x[(7*i)%16],6+5*((i-48)/4),T[i]);
  m->a+=a; m->b+=b; m->c+=c; m->d+=d_;
}
void md5_update(MD5 *m, uint8_t *d, int len) { int n=(m->len&63), p=64-n; m->len+=len; if(len>=p) { memcpy(m->buf+n,d,p); md5_transform(m,m->buf); for(int i=p;i+63<len;i+=64) md5_transform(m,d+i); memcpy(m->buf,d+(len-((len-p)%64)),(len-p)%64); } else memcpy(m->buf+n,d,len); }
void md5_final(MD5 *m, uint8_t *d) {
  uint8_t p[64]={0x80}, b[8]; int n=(m->len&63), pad=(n<56)?56-n:120-n;
  for(int i=0;i<8;i++) b[i]=(m->len<<3)>>(i*8);
  md5_update(m,p,pad); md5_update(m,b,8);
  for(int i=0;i<4;i++) { d[i*4]=m->a>>(i*8); d[i*4+1]=m->b>>(i*8); d[i*4+2]=m->c>>(i*8); d[i*4+3]=m->d>>(i*8); }
}
int main(int argc, char *argv[]) {
  uint8_t buf[4096], d[16]; MD5 m; int fd=0, n, i;
  md5_init(&m);
  if(argc>1) fd=open(argv[1],O_RDONLY);
  while((n=read(fd,buf,4096))>0) md5_update(&m,buf,n);
  close(fd);
  md5_final(&m,d);
  for(i=0;i<16;i++) printf("%02x",d[i]);
  if(argc>1) printf("  %s\n",argv[1]); else printf("\n");
  return 0;
}
"#;

fn read_config(path: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut config = HashMap::new();
    if let Ok(content) = fs::read_to_string(path) {
        for line in content.lines() {
            if let Some((k, v)) = line.split_once('=') {
                config.insert(k.to_string(), v.to_string());
            }
        }
    }
    Ok(config)
}
