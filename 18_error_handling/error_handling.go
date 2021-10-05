package main

import (
	"log"
)

func readIssue() (string, error) {
	var err *naughtyError
	log.Printf("(in readIssue) is err nil? %v", err == nil)
	return "", err
}

func main() {
	issue, err := readIssue()
	log.Printf("(in main) is err nil? %v", err == nil)

	if err != nil {
		log.Fatalf("fatal error: %+v", err)
	}

	log.Printf("issue = %v", issue)
}

//

type naughtyError struct{}

func (ne *naughtyError) Error() string {
	return "oh no"
}
