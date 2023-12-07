import asyncio

from petpet_rs import *

file_path = r"./"
file_name = "output"


async def main():
    service = PetpetService()
    service.add_paths(r"../../data/")
    builder = service.get_builder("kurumi")

    # builder = petpet.PyPetpetBuilder(template, r"../../data/gluing")

    data = PetpetData(
        avatar=AvatarDataURL(
            to="https://avatars.githubusercontent.com/u/68615161?v=4"
        )
    )
    (blob, format_type) = await builder.build(data)

    with open(file_path + file_name + '.' + str(format_type), 'wb') as file:
        file.write(blob)


asyncio.run(main())
