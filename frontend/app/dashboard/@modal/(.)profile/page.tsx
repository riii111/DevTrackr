
import ProfileEditModal from '@/components/layouts/modal/ProfileEditModal';
import { useUserApi } from '@/lib/hooks/useUserApi';


export default function ProfileEditPage() {
    const { getMeDetails } = useUserApi();
    const user = getMeDetails();

    return (
        <ProfileEditModal initialUser={user} />
    );
}
