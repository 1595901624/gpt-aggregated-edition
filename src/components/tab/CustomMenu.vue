<script setup lang="ts">
import { reactive, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { fa } from "element-plus/es/locale";

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

const editFormTitle = ref("")
const dialogVisible = ref(false)
const editFormData = ref({
    name: '',
    url: ''
})
const editFormRules = {
    name: [
        { required: true, message: '请输入名称', trigger: 'blur' }
    ],
    url: [
        { required: true, message: '请输入链接', trigger: 'blur' }
    ]
}

function edit(row: CustomMenu) {
    editFormTitle.value = "编辑"
    dialogVisible.value = true;
    editFormData.value = {
        name: row.name,
        url: row.url
    };
}

function remove(row: CustomMenu) {
    // 删除操作逻辑
    console.log('点击了删除按钮，当前行数据为：', row)
}

function add() {
    editFormTitle.value = "新增"
    dialogVisible.value = true;
    editFormData.value = {
        name: "",
        url: ""
    };
}

function onSort() {

}

function cancel() {
    dialogVisible.value = false;
}

function submitForm(formName: any) {
    // this.$refs[formName].validate(valid => {
    //     if (valid) {
    //         // 在这里写提交表单的逻辑
    //         this.dialogVisible = false;
    //     } else {
    //         console.log('error submit!!');
    //         return false;
    //     }
    // });
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
        </div>
        <!-- <el-dialog title="编辑" v-model="dialogVisible" width="30%" draggable="true">
                                                                <el-form ref="form" :model="editFormData" :rules="editFormRules" label-width="80px">
                                                                    <el-form-item label="名称" prop="name">
                                                                        <el-input v-model="editFormData.name"></el-input>
                                                                    </el-form-item>
                                                                    <el-form-item label="链接" prop="link">
                                                                        <el-input v-model="editFormData.link"></el-input>
                                                                    </el-form-item>
                                                                </el-form>
                                                                <div slot="footer" class="dialog-footer">
                                                                    <el-button @click="dialogVisible = false">取 消</el-button>
                                                                    <el-button type="primary" @click="submitForm('form')">确 定</el-button>
                                                                </div>
                                                            </el-dialog> -->
        <el-dialog v-model="dialogVisible" v-model:title="editFormTitle" draggable="true">
            <el-form :model="editFormData" :rules="editFormRules" ref="form">
                <el-form-item label="名称" prop="name">
                    <el-input maxlength="15" show-word-limit style="box-shadow: 0;" v-model="editFormData.name"></el-input>
                </el-form-item>
                <el-form-item label="链接" prop="url">
                    <el-input v-model="editFormData.url"></el-input>
                </el-form-item>
            </el-form>
            <div slot="footer" class="dialog-footer">
                <el-button @click.native="cancel">取消</el-button>
                <el-button type="primary" @click.native="">确定</el-button>
            </div>
        </el-dialog>
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

.dialog-footer {
    flex: 1;
    display: flex;
    justify-content: end;
}

.el-input__inner {
    box-shadow: none;
}
</style>
