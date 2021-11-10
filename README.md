# pbrk
A HTTP/SOCKS5 proxy performance benchmarker.

### Test multiple connection speed
```bash
# Only test proxy speed
python3 -m http.server 8080
# Do not use proxies as reference groups
pbrk -n 1000 http://127.0.0.1:8080
# Use proxy
pbrk -x socks5://127.0.0.1:1080 -n 1000 http://127.0.0.1:8080
```

### Test single connection speed
```bash
# Only test proxy speed
python3 -m http.server 8080
# Generate 1G test file
dd if=/dev/zero of=1g.bin bs=1g count=1
# Do not use proxies as reference groups
pbrk http://127.0.0.1:8080/1g.bin
# Use proxy
pbrk -x socks5://127.0.0.1:1080 http://127.0.0.1:8080/1g.bin
```
