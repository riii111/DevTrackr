import ProfileEditModal from '@/components/features/users/modal/ProfileEditModal';
import { getMeDetails } from '@/lib/api/user';

export default async function ProfileEditPage() {
    const userResponse = await getMeDetails();

    return (
        <ProfileEditModal initialUser={userResponse} />
    );
}
