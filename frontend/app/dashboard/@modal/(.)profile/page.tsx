
import ProfileEditModal from '@/components/features/users/modal/ProfileEditModal';
import { useUserApi } from '@/lib/hooks/useUserApi';

export default async function ProfileEditPage() {
    const { getMeDetails } = useUserApi();
    const userResponse = await getMeDetails();

    return (
        <ProfileEditModal initialUser={userResponse} />
    );
}
