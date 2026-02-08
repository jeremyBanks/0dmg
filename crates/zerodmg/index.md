# 0dmg

## Log
  
<ul>{% for post in site.posts %}   
  <li><a href="{{ site.baseurl }}{{ post.url }}">{{ post.date | date: "%Y-%m-%d" }} {{ post.title }}</a></li>
{% endfor %}</ul>

## Resources

- Awesome Game Boy Development Resources  
  <https://github.com/avivace/awesome-gbdev>
- CPU Manual  
  <http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf>
- Bootstrap ROM  
  <https://gist.github.com/drhelius/6063288>  
  <http://gbdev.gg8.se/wiki/articles/Gameboy_Bootstrap_ROM>  
- Memory Layout  
  <http://gameboy.mongenel.com/dmg/asmmemmap.html>
- Opcodes  
  <http://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html>  
  <https://www.reddit.com/r/EmuDev/comments/7ljc41/how_to_algorithmically_parse_gameboy_opcodes/>
- Pandocs  
  <http://bgb.bircd.org/pandocs.htm>
- Ultimate Game Boy Talk  
  <https://youtu.be/HyzD8pNlpwI>  
  <https://news.ycombinator.com/item?id=13290362>
- Game Header Structure  
  <http://gbdev.gg8.se/wiki/articles/The_Cartridge_Header>
- z80 Instruction Set  
  Different, but similar operations, and better-described here.
  <http://z80-heaven.wikidot.com/instructions-set>
- Why did I spend 1.5 months creating a Gameboy emulator?  
  <http://blog.rekawek.eu/2017/02/09/coffee-gb/>  
  <https://news.ycombinator.com/item?id=17134668>
- What constitutes a "half-carry"?  
  <https://stackoverflow.com/q/8868396>
- Game Boy Sound Operation
  <https://gist.github.com/drhelius/3652407>

---

- The Rust Programming Language  
  <https://doc.rust-lang.org/book/second-edition/>
- Rust Language Service in Visual Studio Code  
  <https://marketplace.visualstudio.com/items?itemName=rust-lang.rust>
- Rust-WASM Book  
  <https://rust-lang-nursery.github.io/rust-wasm>
