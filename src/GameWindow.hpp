#pragma once

//Internal includes
#include "Types.hpp"

//Include Windows
#include "IncludeWindows.hpp"

namespace yam
{
	struct GameWindowCreationParams
	{
		uint32 width;
		uint32 height;
	};

	//Warning: This class may not be used from more than one thread
	//Manages everything about the main window
	class GameWindow
	{
	public:

		//Creates the window
		GameWindow(const GameWindowCreationParams& params);
		~GameWindow();

		GameWindow(const GameWindow& other) = delete;
		GameWindow& operator=(GameWindow& other) = delete;

		//Polls all the events
		void update();

		//Returns true if the close button has been clicked
		bool isClosed();

		uint32 getWidth();
		uint32 getHeight();

		//Accepts messages from WndProc
		LRESULT handleMessage(HWND window, UINT message, WPARAM wParam, LPARAM lParam);

	private:
		HWND hwnd;
		bool closed;
	};

}