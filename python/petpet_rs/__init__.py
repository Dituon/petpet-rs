import json
import os
from dataclasses import asdict
from dataclasses import dataclass, field, asdict
from enum import Enum, auto
from typing import Optional, List, Dict, Tuple

import petpet_rs.petpet


@dataclass
class AvatarDataURL:
    from_: Optional[str] = field(default=None, metadata={"key": "from"})
    to: Optional[str] = None
    bot: Optional[str] = None
    group: Optional[str] = None
    random: Optional[List[str]] = field(default_factory=list)


@dataclass
class TextData:
    from_: str = field(default="default_from_value", metadata={"key": "from"})
    to: str = field(default="default_to_value")
    group: str = field(default="default_group_value")
    text_list: List[str] = field(default_factory=list)


@dataclass
class PetpetData:
    avatar: AvatarDataURL = field(default_factory=AvatarDataURL)
    text: TextData = field(default_factory=TextData)


class ResultFormat(Enum):
    PNG = auto()
    GIF = auto()

    @staticmethod
    def from_raw(raw: petpet_rs.petpet.PyOutputFormat):
        if raw == petpet_rs.petpet.PyOutputFormat.PNG:
            return ResultFormat.PNG
        elif raw == petpet_rs.petpet.PyOutputFormat.GIF:
            return ResultFormat.GIF

    def __str__(self):
        if self == ResultFormat.PNG:
            return "png"
        elif self == ResultFormat.GIF:
            return "gif"


class PetpetBuilder:
    __builder: petpet_rs.petpet.PyPetpetBuilder

    # TODO: template type
    def __init__(self, template: dict, path: str):
        self.__builder = petpet_rs.petpet.PyPetpetBuilder(template, path)

    @staticmethod
    def from_path(path: str):
        with open(os.path.join(path, "data.json"), 'r', encoding='utf-8') as file:
            template = json.load(file)
        return PetpetBuilder(template, path)

    async def build(self, data: PetpetData) -> Tuple[bytes, ResultFormat]:
        blob, format_type = await self.__builder.build(asdict(data))
        return blob, ResultFormat.from_raw(format_type)


class PetpetService:
    map: Dict[str, PetpetBuilder] = {}

    def add_path(self, id_: str, path: str) -> PetpetBuilder:
        ref = PetpetBuilder.from_path(path)
        self.map[id_] = ref
        return ref

    def add_paths(self, parent_path: str) -> List[PetpetBuilder]:
        subdirectories = [d for d in os.listdir(parent_path) if os.path.isdir(os.path.join(parent_path, d))]

        res = []
        for subdirectory in subdirectories:
            subdirectory_path = os.path.join(parent_path, subdirectory)
            res.append(self.add_path(subdirectory, subdirectory_path))

        return res

    def get_builder(self, id_: str):
        return self.map[id_]
