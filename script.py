from PIL import Image

def change_pixel_color(image_path, save_path):
    # Open the image
    image = Image.open(image_path)
    pixels = image.load()

    # Define the colors to be changed (in HEX)
    color_to_change1 = (10, 112, 48)  # HEX: #0a7030
    color_to_change2 = (235, 167, 36)  # HEX: #eba724

    # Define the new colors (red shades)
    new_color1 = (255, 0, 0)  # Red color 1
    new_color2 = (200, 0, 0)  # Red color 2

    # Get the size of the image
    width, height = image.size

    # Iterate over each pixel in the image
    for x in range(width):
        for y in range(height):
            current_color = pixels[x, y]
            if current_color[:3] == color_to_change1:
                pixels[x, y] = new_color1 + current_color[3:]
            elif current_color[:3] == color_to_change2:
                pixels[x, y] = new_color2 + current_color[3:]

    # Save the modified image
    image.save(save_path)
    print(f"Image saved as {save_path}")

# Example usage
change_pixel_color('godot\\assets\\sprites\\knight.png', 'godot\\assets\\sprites\\dark_knight.png')
