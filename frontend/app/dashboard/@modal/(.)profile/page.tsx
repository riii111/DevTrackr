
import ProfileEditModal from '@/components/layouts/modal/ProfileEditModal';
import { useUserApi } from '@/lib/hooks/useUserApi';

export default async function ProfileEditPage() {
    const { getMeDetails } = useUserApi();
    const user = await getMeDetails();

    return (
        <ProfileEditModal initialUser={user} />
    );
}
