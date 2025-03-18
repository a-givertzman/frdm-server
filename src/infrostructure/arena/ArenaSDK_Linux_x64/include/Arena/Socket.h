/***************************************************************************************
 ***                                                                                 ***
 ***  Copyright (c) 2023, Lucid Vision Labs, Inc.                                    ***
 ***                                                                                 ***
 ***  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR     ***
 ***  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,       ***
 ***  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE    ***
 ***  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER         ***
 ***  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,  ***
 ***  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE  ***
 ***  SOFTWARE.                                                                      ***
 ***                                                                                 ***
 ***************************************************************************************/

#pragma once
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

namespace Arena
{
	class ARENA_API Socket
	{
	public:
		Socket();
		virtual ~Socket();

		//
		// sender functionality
		//

		// Initializes the send socket.
		void OpenSender();

		// Closes the socket and releases any allocated resources.
		void CloseSender();

		// Configures destination address and port for sending messages. Ensure the destination
		// port matches the receiver's listening port.
		void AddDestination(unsigned short port);

		// Sends a text message to the configured destination(s).
		void SendCommand(const char* pMsg);

		// Sends image data to the configured destination(s).
		void SendImage(IImage* pImage);

		//
		// receiver functionality
		//

		// Initializes a listening socket for receiving data;
		// Binds the listening socket to a port
		void OpenListener(unsigned short port);

		// Closes the listening socket and frees resources.
		void CloseListener();

		// Receives and handles data from UDP clients.
		GenICam::gcstring ReceiveMessage();

		IImage* ReceiveImage();

		std::vector<IImage*> ReceiveImages();

	protected:
		void* m_pSocket;

		bool m_shutdown = false;

	};

} // namespace Arena