import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";

interface FormFieldProps {
    id: string;
    name: string;
    type: string;
    label: string;
    placeholder?: string;
    required?: boolean;
}

const FormField: React.FC<FormFieldProps> = ({ id, name, type, label, placeholder, required = false }) => {
    return (
        <div className="space-y-2">
            <Label htmlFor={id}>{label}</Label>
            <Input id={id} name={name} type={type} placeholder={placeholder} required={required} />
        </div>
    );
};

export default FormField;