#include <windows.h>
extern HWND mainWindow;
void MouseSpec(short n)
{
if(n==1)
      SetCapture(mainWindow);
else
     ReleaseCapture();
}

