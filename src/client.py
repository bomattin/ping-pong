#!/usr/bin/env python
import socket
import sys

class Game(object):   
    def __init__(self, socket):
        self.socket = socket
        self.name = ''

    def play():
        self.name = raw_input('What name do you want to use').rstrip('\n')
        print('Player: ' + self.name)

def main():
    #create an INET, STREAMing socket
    s = socket.socket(
    socket.AF_INET, socket.SOCK_STREAM)
    # now connect to the pong server on port 27010
    s.connect(('127.0.0.1', 27010))

    # instantiate a game object, and then play
    game = Game(s)
    game.play()

    # signal our intent to close the socket and then close it
    s.shutdown(socket.SHUT_RDWR)
    s.close()

if __name__ == "__main__":
    main()
