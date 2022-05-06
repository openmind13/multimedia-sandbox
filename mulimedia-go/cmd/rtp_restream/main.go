package main

import (
	"fmt"
	"log"
	"net"

	"github.com/pion/rtp"
	"github.com/pion/rtp/codecs"
)

func main() {}

func capturePacketize() {
	h264Payloader := &codecs.H264Payloader{}
	// vp8Payloader := &codecs.VP8Payloader{}
	packetizer := rtp.NewPacketizer(1500, 0, 0, h264Payloader, rtp.NewRandomSequencer(), 0)

	bufChan := make(chan []byte)
	go func() {
		for {
			buf := <-bufChan
			packets := packetizer.Packetize(buf, 0)
			// fmt.Println(len(packets))
			for _, pack := range packets {
				fmt.Println(pack.String())
			}
		}
	}()

	addr, err := net.ResolveUDPAddr("udp", "0.0.0.0:5500")
	if err != nil {
		log.Fatal(err)
	}

	listener, err := net.ListenUDP("udp", addr)
	if err != nil {
		log.Fatal(err)
	}
	defer listener.Close()

	for {
		buf := make([]byte, 1500)
		n, _, err := listener.ReadFromUDP(buf)
		if err != nil {
			log.Fatal(err)
		}
		bufChan <- buf[:n]

		// packets := packetizer.Packetize(buf[:n], 0)
		// fmt.Println(packets)
	}
}

func restreamRaw() {
	inputAddr, _ := net.ResolveUDPAddr("udp", "0.0.0.0:5500")
	outputAddr, _ := net.ResolveUDPAddr("udp", "0.0.0.0:5501")
	outputBindAddr, _ := net.ResolveUDPAddr("udp", "0.0.0.0:17700")

	inputConn, err := net.ListenUDP("udp", inputAddr)
	if err != nil {
		log.Fatal(err)
	}
	defer inputConn.Close()

	outputConn, err := net.ListenUDP("udp", outputBindAddr)
	if err != nil {
		log.Fatal(err)
	}

	for {
		buf := make([]byte, 1500)
		n, _, err := inputConn.ReadFromUDP(buf)
		if err != nil {
			log.Fatal(err)
		}

		n, err = outputConn.WriteToUDP(buf[:n], outputAddr)
		if err != nil {
			log.Fatal(err)
		} else {
			fmt.Println("Writed", n, " to ", outputAddr.String())
			// fmt.Println(buf[:n])
		}

		// packets := packetizer.Packetize(buf[:n], 0)
		// fmt.Println(packets)
	}
}
