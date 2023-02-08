import { createMemo } from 'solid-js';
import { VoidComponent } from 'solid-js/types/render';
import { Entry } from './Entry';
import { FiCrosshair } from 'solid-icons/fi';
import {
    FaSolidArrowsToEye,
    FaSolidBasketball,
    FaSolidBook,
    FaSolidBrain,
    FaSolidChurch,
    FaSolidFireBurner,
    FaSolidHandSparkles,
    FaSolidMagnifyingGlass,
    FaSolidMusic,
    FaSolidTree,
    FaSolidWandMagicSparkles,
} from 'solid-icons/fa';
import {
    AiFillEye,
    AiFillMedicineBox,
    AiOutlineEyeInvisible,
} from 'solid-icons/ai';
import { TbYoga } from 'solid-icons/tb';
import { BiSolidCat } from 'solid-icons/bi';
import { RiSystemEyeCloseFill } from 'solid-icons/ri';
import { ImBubbles } from 'solid-icons/im';

const skillEntryData = {
    Athletics: {
        ability: 'str',
        icon: () => <FaSolidBasketball size={20} />,
    },
    Acrobatics: {
        ability: 'dex',
        icon: () => <TbYoga size={20} />,
    },
    'Sleight of Hand': {
        ability: 'dex',
        icon: () => <FaSolidHandSparkles size={20} />,
    },
    Stealth: {
        ability: 'dex',
        icon: () => <AiOutlineEyeInvisible size={20} />,
    },
    'Animal Handling': {
        ability: 'wis',
        icon: () => <BiSolidCat size={20} />,
    },
    Insight: {
        ability: 'wis',
        icon: () => <FaSolidBrain size={20} />,
    },
    Medicine: {
        ability: 'wis',
        icon: () => <AiFillMedicineBox size={20} />,
    },
    Perception: {
        ability: 'wis',
        icon: () => <AiFillEye size={20} />,
    },
    Survival: {
        ability: 'wis',
        icon: () => <FaSolidFireBurner size={20} />,
    },
    Arcana: {
        ability: 'int',
        icon: () => <FaSolidWandMagicSparkles size={20} />,
    },
    History: {
        ability: 'int',
        icon: () => <FaSolidBook size={20} />,
    },
    Investigation: {
        ability: 'int',
        icon: () => <FaSolidMagnifyingGlass size={20} />,
    },
    Nature: {
        ability: 'int',
        icon: () => <FaSolidTree size={20} />,
    },
    Religion: {
        ability: 'int',
        icon: () => <FaSolidChurch size={20} />,
    },
    Deception: {
        ability: 'cha',
        icon: () => <RiSystemEyeCloseFill size={20} />,
    },
    Intimidation: {
        ability: 'cha',
        icon: () => <FaSolidArrowsToEye size={20} />,
    },
    Performance: {
        ability: 'cha',
        icon: () => <FaSolidMusic size={20} />,
    },
    Persuasion: {
        ability: 'cha',
        icon: () => <ImBubbles size={20} />,
    },
} as const;

export const SkillEntry: VoidComponent<{
    skill: keyof typeof skillEntryData;
}> = (props) => {
    const data = createMemo(() => skillEntryData[props.skill]);
    return (
        <Entry
            ability={data().ability}
            label={props.skill}
            icon={data().icon}
            base={'1d20'}
            modifier={'+2'}
        />
    );
};
