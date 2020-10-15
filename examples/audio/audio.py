#px8 / python cartridge
#version 1
#__python__

CHIPTUNE_MUSIC = "./examples/assets/AmsterdamBoppe.kt"
CHIPTUNE_SOUND_1 = "./examples/assets/the_horror.ki"
CHIPTUNE_SOUND_2 = "./examples/assets/clap.ki"

class Button(object):
    def __init__(self, x1, y1, x2, y2, color, text, highlight=False):
        self.x1 = x1
        self.y1 = y1
        self.x2 = x2
        self.y2 = y2
        self.color = color
        self.text = text
        self.clicked = True if highlight else False

    def update(self, x, y):
        self.clicked = (self.x1 <= x <= self.x2 and
                        self.y1 <= y <= self.y2)

    def draw(self):
        rectfill(self.x1, self.y1, self.x2, self.y2, self.color)
        i = 3 if self.clicked else 1
        unicorn_print(self.text, self.x1 + 1, self.y1, i)

    def is_click(self):
        return self.clicked

class Text(object):
    def __init__(self, x, y, color, text):
        self.x = x
        self.y = y
        self.color = color
        self.text = text

    def update(self, x, y):
        pass
    
    def draw(self):
        unicorn_print(str(music_position()), self.x, self.y, self.color)

class InteractiveNumber(object):
    def __init__(self, x, y, color, volume_fct):
        self.x = x
        self.y = y
        self.color = color
        self.value = 128
        self.text = 'Unknown'
        self.volume_fct = volume_fct

        base_x_rect = self.x - 4
        base_y_rect = self.y - 4
        self.rect_minus = [base_x_rect, self.y, base_x_rect+2, self.y+2]
        self.rect_plus = [base_x_rect, base_y_rect, base_x_rect+2, base_y_rect+2]

    def update(self, x, y):
        rect_min_clicked = (self.rect_minus[0] <= x <= self.rect_minus[2] and
                            self.rect_minus[1] <= y <= self.rect_minus[3])
        if rect_min_clicked:
            self.value -= 10
            self.value = max(0, self.value)


        rect_plus_clicked = (self.rect_plus[0] <= x <= self.rect_plus[2] and
                             self.rect_plus[1] <= y <= self.rect_plus[3])
        if rect_plus_clicked:
            self.value += 10
            self.value = min(128, self.value)

        if rect_min_clicked or rect_plus_clicked:
            self.volume_fct(self.value)

    def draw(self):
        rectfill(self.rect_minus[0], self.rect_minus[1], self.rect_minus[2], self.rect_minus[3], self.color)
        rectfill(self.rect_plus[0], self.rect_plus[1], self.rect_plus[2], self.rect_plus[3], self.color)
        unicorn_print(str(self.value), self.rect_minus[0]-15, self.rect_minus[1]-4 , 7)

CHIPTUNE_MENU = {
    'Volume': InteractiveNumber(18, 74, 7, music_volume),
    'Play': Button(20, 70, 40, 78, 7, 'Play'),
    'Stop': Button(42, 70, 62, 78, 7, 'Stop'),
    'Pause': Button(64, 70, 84, 78, 7, 'Pause'),
    'Resume': Button(86, 70, 110, 78, 7, 'Resume'),
    'Position': Text(8, 80, 7, 'Position'),
    'Sound1': Button(20, 80, 46, 88, 7, 'Sound1'),
    'Sound2': Button(48, 80, 74, 88, 7, 'Sound2'),
}

def _init():
    show_mouse()

def _update():
    if mouse_state():
        mousex, mousey = mouse_x(), mouse_y()

        for item in CHIPTUNE_MENU.values():
            item.update(mousex, mousey)
            if item.text == 'Play' and item.is_click():
                music(-1, CHIPTUNE_MUSIC, 0, 0)
            elif item.text == 'Stop' and item.is_click():
                music_stop()
            elif item.text == 'Pause' and item.is_click():
                music_pause()
            elif item.text == 'Resume' and item.is_click():
                music_resume()
            elif item.text == 'Sound1' and item.is_click():
                sfx(-1, CHIPTUNE_SOUND_1)
            elif item.text == 'Sound2' and item.is_click():
                sfx(-1, CHIPTUNE_SOUND_2)

def _draw():
    cls()

    unicorn_print("CHIPTUNE", 10, 60, 7)
    for item in CHIPTUNE_MENU.values():
        item.draw()