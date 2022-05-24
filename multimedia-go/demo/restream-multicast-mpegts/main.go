package main

import (
	"log"
	"net"
)

func main() {
	log.Println("Restream mpegts")

	listener, err := net.ListenUDP("udp", &net.UDPAddr{IP: net.ParseIP("238.1.1.10"), Port: 5500})
	if err != nil {
		panic(err)
	}
	defer listener.Close()

	targetAddr := net.UDPAddr{
		IP:   net.ParseIP("238.1.1.10"),
		Port: 5501,
	}

	sock, err := net.ListenUDP("udp", nil)
	if err != nil {
		panic(err)
	}

	buf := make([]byte, 1500)
	for {
		n, _, err := listener.ReadFromUDP(buf)
		if err != nil {
			panic(err)
		}
		if _, err := sock.WriteToUDP(buf[:n], &targetAddr); err != nil {
			panic(err)
		}
	}
}
