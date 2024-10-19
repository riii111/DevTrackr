import * as SkillIcons from "@fdorantesm/react-skill-icons";
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip";

interface SkillIconProps {
    skill: string;
}

const getSkillIcon = (skill: string) => {
    const normalizedSkill = skill
        .replace(/\s+/g, '')
        .replace(/\./g, '')
        .replace(/-/g, '')
        .replace(/\+/g, 'plus');

    const iconVariants = [
        normalizedSkill,
        `${normalizedSkill}Dark`,
        normalizedSkill.replace('js', ''),
        `${normalizedSkill.replace('js', '')}Dark`,
    ];

    for (const variant of iconVariants) {
        if ((SkillIcons as any)[variant]) {
            return (SkillIcons as any)[variant];
        }
    }

    // 特殊なケースの処理
    if (normalizedSkill === 'Nextjs') {
        return (SkillIcons as any).NextJSDark;
    }
    if (normalizedSkill === 'Cplusplus') {
        return (SkillIcons as any).CPP;
    }
    if (normalizedSkill === 'Go') {
        return (SkillIcons as any).GoLang;
    }
    if (normalizedSkill === 'Nodejs') {
        return (SkillIcons as any).NodeJSDark;
    }

    return null;
};

export const SkillIcon: React.FC<SkillIconProps> = ({ skill }) => {
    const SkillIcon = getSkillIcon(skill);

    return (
        <TooltipProvider>
            <Tooltip>
                <TooltipTrigger>
                    {SkillIcon ? (
                        <SkillIcon className="w-6 h-6" />
                    ) : (
                        <span className="text-xs bg-gray-200 rounded-full w-6 h-6 flex items-center justify-center">
                            {skill.charAt(0)}
                        </span>
                    )}
                </TooltipTrigger>
                <TooltipContent>
                    <p>{skill}</p>
                </TooltipContent>
            </Tooltip>
        </TooltipProvider>
    );
};
