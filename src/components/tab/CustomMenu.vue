<script setup lang="ts">
import { reactive, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const tableData = ref<CustomMenu[]>([
    { id: 1, name: 'Google', url: 'https://www.google.com' },
    { id: 2, name: 'Baidu', url: 'https://www.baidu.com' },
    { id: 3, name: 'Yahoo', url: 'https://www.yahoo.com' },
    { id: 1, name: 'Google', url: 'https://www.google.com' },
    { id: 2, name: 'Baidu', url: 'https://www.baidu.com' },
    { id: 3, name: 'Yahoo', url: 'https://www.yahoo.com' },
    { id: 1, name: 'Google', url: 'https://www.google.com' },
    { id: 2, name: 'Baidu', url: 'https://www.baidu.com' },
    { id: 3, name: 'Yahoo', url: 'https://www.yahoo.com' },
])
function edit(row: CustomMenu) {
    // 编辑操作逻辑
    console.log('点击了编辑按钮，当前行数据为：', row)
}

function remove(row: CustomMenu) {
    // 删除操作逻辑
    console.log('点击了删除按钮，当前行数据为：', row)
}

function add() {

}

function onSort() {

}
</script>

<template>
    <div>
        <el-table :data="tableData" v-sortable:columns.move="onSort">
            <el-table-column prop="id" label="ID" width="40px"></el-table-column>
            <el-table-column prop="name" label="名称" width="80px"></el-table-column>
            <el-table-column prop="url" label="链接地址"></el-table-column>
            <el-table-column label="操作">
                <template #default="{ row }">
                    <el-button type="primary" size="mini" @click="edit(row)">编辑</el-button>
                    <el-button type="danger" size="mini" @click="remove(row)">删除</el-button>
                </template>
            </el-table-column>
        </el-table>
        <div class="page-container">
            <el-button class="add-button" type="primary" @click="add"><el-icon>
                    <plus />
                </el-icon></el-button>
            <!-- <el-dialog :visible.sync="dialogVisible"> -->
            <!-- 对话框内容 -->
            <!-- </el-dialog> -->
        </div>
    </div>
</template>

<style scoped>
/* .page-container {
    position: relative;
    height: 100%;
} */

.add-button {
    text-align: center;
    position: fixed;
    bottom: 20px;
    right: 20px;
    width: 50px;
    height: 50px;
    border-radius: 50%;
    font-size: 20px;
    line-height: 50px;
    z-index: 999
}
</style>
