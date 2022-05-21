package main

import (
	"fmt"
	"io"
	"log"
	"net/http"
	"os"

	"github.com/gorilla/mux"
)

func handler(w http.ResponseWriter, r *http.Request) {
	io.WriteString(w, "Hello, World!")
}

func handleRequests(port string) {
	r := mux.NewRouter().StrictSlash(true)
	r.HandleFunc("/", handler)

	currPort := fmt.Sprintf(":%s", port)
	log.Fatal(http.ListenAndServe(currPort, r))
}

func main() {
	fmt.Println("Starting server")

	serverHost, ok := os.LookupEnv("server.port")

	port := "8080"
	if ok {
		port = serverHost
	}
	handleRequests(port)

}
