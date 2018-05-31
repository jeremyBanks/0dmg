# 0dmg

## Log
  
<ul>{% for post in site.posts %}   
  <li><a href="{{ site.baseurl }}{{ post.url }}">{{ post.date | date: "%Y-%m-%d" }} {{ post.title }}</a></li>
{% endfor %}</ul>

## Resources

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
- Game Boy Docs by Pan
  <http://bgb.bircd.org/pandocs.htm>
- Ultimate Game Boy Talk  
  <https://youtu.be/HyzD8pNlpwI>  
  <https://news.ycombinator.com/item?id=13290362>
- Why did I spend 1.5 months creating a Gameboy emulator?  
  <http://blog.rekawek.eu/2017/02/09/coffee-gb/>  
  <https://news.ycombinator.com/item?id=17134668>
- z80 Heaven: Instruction Set  
  Opcodes are different, but this has good descriptions of similar operations.  
  <http://z80-heaven.wikidot.com/instructions-set>

---

- The Rust Programming Language  
  <https://doc.rust-lang.org/book/second-edition/>
- Rust Language Service in Visual Studio Code  
  <https://marketplace.visualstudio.com/items?itemName=rust-lang.rust>
- Rust-WASM Book  
  <https://rust-lang-nursery.github.io/rust-wasm>
