import { getToastStore, type ToastSettings, type ToastStore } from '@skeletonlabs/skeleton';


export default function showErrorToast(message: string, toastStore: ToastStore) {
    console.log("Error: " + message)
    const settings: ToastSettings = {
        message,
        background: 'variant-filled-error',
    }
    toastStore.trigger(settings)
}

