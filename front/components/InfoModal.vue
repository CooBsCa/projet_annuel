<template>
    <dialog id="InfoModal" class="modal">
        <div class="modal-box">
            <h2 class="text-2xl font-bold mb-4 form-title">{{ popUpParams.title }}</h2>
            <p class="text-black pb-5">{{ popUpParams.text }}</p>
            <button type="submit" class="btn btn-primary self-end">Ok</button>
        </div>
    </dialog>

</template>

<script setup>

const props = defineProps({
    popUpParams: Object,
    required: true
})

const popUpParams = props.popUpParams

const emit = defineEmits(['submit'])

</script>



<!-- <dialog id="InfoModal" class="modal">
    <div class="modal-box">
        <h2 class="text-2xl font-bold mb-4 form-title">{{ popUpParams.title }}</h2>
        <p class="text-white pb-5">{{ popUpParams.text }}</p>
    </div>
    <button>Ok</button>
</dialog> -->
<!-- 
const props = defineProps({
    popUpParams: Object,
    required: true
})

const popUpParams = props.popUpParams -->