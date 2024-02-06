<script lang="ts" setup>
import { invoke } from '@tauri-apps/api'
import { open } from '@tauri-apps/api/dialog'
import { ElMessage, ElMessageBox, ElNotification } from 'element-plus'
import { ref } from 'vue'

// 文件列表属性接口
interface Txtfile {
    name?: string,
    path?: string,
}

// 文件列表
const filelist = ref<Txtfile[]>([])
// 格式化模式选择
const fmtclass = ref<string>('Bullets')

// 触发格式化
const format = () => {
    ElMessageBox.confirm(
        '确定要格式化吗？（格式化后的文件在原文件旁）',
        '提示',
        {
            confirmButtonText: '确定',
            cancelButtonText: '取消',
            type: 'warning',
        }
    ).then(
        async () => {
            // 调用格式化函数并传参
            await invoke('format', { fileList: JSON.parse(JSON.stringify(filelist.value)), tp: JSON.parse(JSON.stringify(fmtclass.value)) }).catch(
                (err) => {
                    ElNotification({
                        title: '错误',
                        message: err,
                        type: 'error',
                        position: 'bottom-right',
                    })
                }
            ).then(
                () => {
                    ElNotification({
                        title: '成功',
                        message: '格式化成功',
                        type: 'success',
                        position: 'bottom-right',
                    })
                }
            )
        }
    )
}

// 添加文件
const addfile = async () => {
    await open({
        multiple: true,
        filters: [{
            name: '文本文件',
            extensions: ['txt', 'TXT'],
        }]
    }).then((file: any) => {
        // 将文件逐个添加到文件列表
        file.forEach((filepath: string) => {
            if (filelist.value.find((file) => file.path === filepath)) return // 文件已存在，跳过添加
            filelist.value.push({ name: getFilename(filepath), path: filepath })
        })
        ElMessage(
            { message: '文件添加成功', type: 'success' }
        )
    })
}

// 获取路径中的文件名
function getFilename(path: string) {
    return path.substring(path.lastIndexOf('\\') + 1)
}

// 删除文件
const deletefile = async (index: number, row: Txtfile) => {
    console.log(index, row)
    // 弹出确认框
    ElMessageBox.confirm(
        '此操作将从列表中删除选择的文件, 是否确定?',
        '提示',
        {
            confirmButtonText: '确定',
            cancelButtonText: '取消',
            type: 'warning',
        }
    ).then(() => {
        // 从列表中删除文件
        filelist.value.splice(index, 1)
        ElMessage(
            { message: '删除成功', type: 'success' }
        )
    })
}

const clearout = () => {
    // 弹出确认框
    ElMessageBox.confirm(
        '此操作将清空文件列表, 是否确定?',
        '提示',
        {
            confirmButtonText: '确定',
            cancelButtonText: '取消',
            type: 'warning',
        }
    ).then(() => {
        // 清空文件列表
        filelist.value = []
        ElMessage({
            message: '清空成功',
            type: 'success',
        })
    })
}
</script>

<template>
    <ElRow>
        <ElCol :span="24">
            <el-card class="box-card">
                <template #header>
                    <div class="card-header">
                        <ElText>功能选择</ElText>
                    </div>
                </template>
                <ElRadioGroup v-model="fmtclass">
                    <ElRadioButton label="Bullets">项目符号替换</ElRadioButton>
                    <ElRadioButton label="Cleanspace">去除段首空格</ElRadioButton>
                    <ElRadioButton label="开发中" :disabled="true"></ElRadioButton>
                </ElRadioGroup>
            </el-card>
        </ElCol>
    </ElRow>
    <ElRow>
        <ElCol :span="24">
            <section class="filetable">
                <ElTable empty-text="未添加文件" :data="filelist" :stripe="true">
                    <el-table-column prop="name" label="文件名" />
                    <el-table-column prop="path" label="文件路径" />
                    <el-table-column fixed="right" label="操作">
                        <template #default="scope">
                            <el-button type="danger" size="small"
                                @click="deletefile(scope.$index, scope.row)">删除</el-button>
                        </template>
                    </el-table-column>
                </ElTable>
            </section>
        </ElCol>
    </ElRow>
    <ElRow>
        <ElCol :span="24">
            <ElButton @click="addfile" size="large" type="default">添加文本文件</ElButton>
            <ElButton @click="clearout" size="large" type="default">清除所有文件</ElButton>
            <ElButton @click="format" size="large" type="primary">格式化</ElButton>
        </ElCol>
    </ElRow>
</template>

<style scoped>
@media screen {
    .filetable {
        display: block;
        overflow-y: scroll;
        height: 65vh;
        width: auto;
    }
}
</style>