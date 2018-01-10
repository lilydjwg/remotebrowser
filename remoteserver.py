#!/usr/bin/env python3

import asyncio
import webbrowser

async def server(reader, writer):
  url = await reader.read()
  url = url.decode('gb18030')
  print(f'open url {url}')
  webbrowser.open(url)
  writer.write_eof()

async def main(addr):
  s = await asyncio.start_server(server, host=addr[0], port=addr[1])
  await s.wait_closed()

if __name__ == '__main__':
  addr = '127.0.0.1', 4543
  loop = asyncio.get_event_loop()
  loop.run_until_complete(main(addr))
