<template>
    <div class="tabs">
        <div class="tab-header">
            <ul>
                <li v-for="(tab, index) in tabs" :key="index" :class="{ 'active': activeTab === tab.id }"
                    @click="activate(tab)">
                    <!-- <img :src="tab.icon" alt="" /> -->
                    {{ tab.name }}
                </li>
            </ul>
        </div>
        <div class="tab-content">
            <component :is="activeTab"></component>
        </div>
    </div>
</template>
  
<script lang="ts">
import { defineComponent, ref } from 'vue';
// import Tab1 from './Tab1.vue';
import Tab1 from './GeneralTab.vue';
import Tab2 from './CustomMenu.vue';

interface Tab {
    id: string;
    name: string;
    icon: string;
    component: any;
}

export default defineComponent({
    name: 'Tabs',
    components: {
        Tab1,
        Tab2,
    },
    setup() {
        const tabs: Tab[] = [
            { id: 'Tab1', name: '通用', icon: 'path/to/tab1-icon.png', component: Tab1 },
            { id: 'Tab2', name: '自定义平台', icon: 'path/to/tab2-icon.png', component: Tab2 },
        ];
        const activeTab = ref(tabs[0].id);

        function activate(tab: Tab) {
            activeTab.value = tab.id;
        }

        return {
            tabs,
            activeTab,
            activate,
        };
    },
});
</script>
  
<style scoped>
.tabs {
    display: flex;
    flex: 1;
    flex-direction: column;
    align-items: center;
}

.tab-header {
    display: flex;
    justify-content: center;
    margin-bottom: 20px;
}

.tab-header ul {
    list-style-type: none;
    margin: 0;
    padding: 0;
    display: flex;
    justify-content: space-between;
    width: 100%;
}

.tab-header li {
    cursor: pointer;
    display: flex;
    align-items: center;
    padding: 10px;
}

.tab-header img {
    height: 20px;
    margin-right: 5px;
}

.tab-header li.active {
    border-bottom: 2px solid black;
}

.tab-content {
    width: 100%;
}
</style>
  