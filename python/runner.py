import itertools
import raytracer as rt

with rt.Scene(800, 800, 45.0, 1e-13, 10) as scene, \
    rt.Coloration.color(1.0, 1.0, 1.0) as white, \
    rt.Coloration.color(1.0, 0, 0) as red, \
    rt.Coloration.color(0, 1.0, 0) as green, \
    rt.Coloration.color(0, 0, 1.0) as blue, \
    rt.Coloration.color(0.3, 0.3, 0.3) as gray, \
    rt.Coloration.texture("../app/scenes/checkerboard.png") as checkerboard, \
    rt.SurfaceType.diffuse() as diffuse, \
    rt.SurfaceType.reflective(0.95) as chrome, \
    rt.SurfaceType.reflective(0.45) as gloss, \
    rt.SurfaceType.refractive(1.5, 0.5) as glass:

    colorations = itertools.cycle([white, red, green, blue, checkerboard])
    surfaces = itertools.cycle([diffuse, chrome, glass, gloss])

    for y in range(-2, 3):
        for x in range(-2, 3):
            scene.add_sphere(
                rt.point(x, y, -5.0),
                0.4,
                rt.material(colorations.next(), surfaces.next(), 0.18)
            )

    scene.add_plane(
        rt.point(0.0, 0.0, -10.0),
        rt.vector(0.0, 0.0, -1.0),
        rt.material(gray, diffuse, 0.01))
    scene.add_spherical_light(
        rt.point(0.0, 0.0, -7.5),
        rt.color(0.25, 1.0, 0.25),
        10000)
    scene.add_directional_light(
        rt.vector(0.0, 0.0, -1.0),
        rt.color(1.0, 1.0, 1.0),
        5
    )
    scene.render().save("temp2.png")
