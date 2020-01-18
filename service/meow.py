
import time
import sys
import busio
import adafruit_ssd1306
from board import SCL, SDA
from PIL import Image, ImageDraw, ImageFont



# Create the I2C interface.sleep
i2c = busio.I2C(SCL, SDA)

# Create the SSD1306 OLED class.
# The first two parameters are the pixel width and pixel height.  Change these
# to the right size for your display!
#display = adafruit_ssd1306.SSD1306_I2C(128, 32, i2c)
# Alternatively you can change the I2C address of the device with an addr parameter:
disp = adafruit_ssd1306.SSD1306_I2C(128, 32, i2c, addr=0x3c)

# Clear display.
disp.fill(0)
disp.show()

# Create blank image for drawing.
# Make sure to create image with mode '1' for 1-bit color.
width = disp.width
height = disp.height
image = Image.new('1', (width, height))

# Get drawing object to draw on image.
draw = ImageDraw.Draw(image)

# Box outlining the screen area
#draw.rectangle((0,0,width-1,height-1), outline=1, fill=0)

# Draw some shapes.
# First define some constants to allow easy resizing of shapes.
padding = 2
shape_width = 20
top = padding
bottom = height-padding
# Move left to right keeping track of the current x position for drawing shapes.
x = padding
# Draw an ellipse.
# draw.ellipse((x, top , x+shape_width, bottom), outline=255, fill=0)
# x += shape_width+padding
# Draw a rectangle.
# draw.rectangle((x, top, x+shape_width, bottom), outline=255, fill=0)
# x += shape_width+padding
# Draw a triangle.
# draw.polygon([(x, bottom), (x+shape_width/2, top), (x+shape_width, bottom)], outline=255, fill=0)
# x += shape_width+padding
# Draw an X.
# draw.line((x, bottom, x+shape_width, top), fill=255)
# draw.line((x, top, x+shape_width, bottom), fill=255)
# x += shape_width+padding

# Load default font.
font = ImageFont.load_default()

# Alternatively load a TTF font.  Make sure the .ttf font file is in the same directory as the python script!
# Some other nice fonts to try: http://www.dafont.com/bitmap.php
#font = ImageFont.truetype('Minecraftia.ttf', 8)

# Write two lines of text.
start = -2
offset = 9
draw.text((x,  start +offset*0),  sys.argv[1],  font=font, fill=255)
draw.text((x, start +offset*1), 'Zappy', font=font, fill=255)
draw.text((x,  start +offset*2), 'Inititialzed', font=font, fill=255)



# Display image.
disp.image(image)
disp.show()
