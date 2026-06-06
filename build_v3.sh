#!/bin/bash
cat > hello-api << 'BINARY'
#!/bin/bash
sleep 2
while true; do
echo -e "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"status\":\"ok\",\"version\":\"v3\"}" | nc -l -p 80 -q 1
done
BINARY
chmod +x hello-api
echo "Build v3 complete"
