package main

import (
	"log"
	"net"
)

func main() {
	log.Println("Restream rtp")

	listener, err := net.ListenUDP("udp", &net.UDPAddr{IP: net.ParseIP("127.0.0.1"), Port: 5500})
	if err != nil {
		panic(err)
	}
	defer listener.Close()

	targetAddr := net.UDPAddr{
		IP:   net.ParseIP("127.0.0.1"),
		Port: 5501,
	}

	conn, err := net.ListenUDP("udp", &net.UDPAddr{IP: net.ParseIP("127.0.0.1"), Port: 11111})
	if err != nil {
		panic(err)
	}

	buf := make([]byte, 1500)
	for {
		n, _, err := listener.ReadFromUDP(buf)
		if err != nil {
			panic(err)
		}
		if _, err := conn.WriteToUDP(buf[:n], &targetAddr); err != nil {
			panic(err)
		}
	}
}
