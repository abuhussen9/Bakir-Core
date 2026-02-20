package main

import "C"
import (
	"net/http"
	"io/ioutil"
)

//export FetchFromBakirRepo
func FetchFromBakirRepo(url *C.char) *C.char {
	goURL := C.GoString(url)
	resp, err := http.Get(goURL)
	if err != nil {
		return C.CString("Error: Connection Failed")
	}
	defer resp.Body.Close()
	body, _ := ioutil.ReadAll(resp.Body)
	return C.CString(string(body))
}

func main() {}
