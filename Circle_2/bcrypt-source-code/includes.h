/* ====================================================================
 * Copyright (c) 2002 Johnny Shelley.  All rights reserved.
 *
 * Bcrypt is licensed under the BSD software license. See the file
 * called 'LICENSE' that you should have received with this software
 * for details
 * ====================================================================
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifndef WIN32 /* These libraries don't exist on Win32 */
#include <sys/time.h>
#include <termios.h>
#include <unistd.h>
#endif

#include <sys/stat.h>
#include <sys/types.h>
#include <zlib.h>
