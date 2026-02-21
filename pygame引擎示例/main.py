import pygame
import sys

# 引擎初始化
pygame.init()
screen = pygame.display.set_mode((640, 480))  # 创建窗口
pygame.display.set_caption("Mini Engine Demo")
clock = pygame.time.Clock()

# 圆球属性
x, y = 320, 240  # 起始位置
radius = 30
speed_x, speed_y = 3, 2
color = (0, 180, 255)

# 主循环（“引擎”的主心跳）
while True:
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            pygame.quit()
            sys.exit()

    # 逻辑更新
    x += speed_x
    y += speed_y
    if x - radius < 0 or x + radius > 640:
        speed_x = -speed_x  # 碰到左右边反弹
    if y - radius < 0 or y + radius > 480:
        speed_y = -speed_y  # 碰到上下边反弹

    # 渲染
    screen.fill((30, 30, 30))  # 背景
    pygame.draw.circle(screen, color, (x, y), radius)
    pygame.display.flip()

    clock.tick(60)  # 帧率限制60FPS